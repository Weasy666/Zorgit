use std::borrow::Cow;
use std::error::Error;
use std::path::Path;
use std::process::Stdio;
use anyhow::anyhow;
use rocket::http::{ContentType, Header};
use rocket::data::DataStream;
use tokio::process::Command;

mod routes;
pub use self::routes::Server;

pub async fn info_refs<P: AsRef<Path>>(repo_path: P, service: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let output = Command::new("git")
        .arg(service)
        .arg("--stateless-rpc")
        .arg("--advertise-refs")
        .arg(repo_path.as_ref())
        .output()
        .await?;

    let packet = format!("# service=git-{}\n", service);
    let length = packet.len() + 4;
    let payload = std::str::from_utf8(&output.stdout)?;

    Ok(format!("{:0>4x}{}{}0000", length, packet, payload).as_bytes().to_vec())
}

pub async fn upload_pack<P: AsRef<Path>>(repo_path: P, change_set: DataStream) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut child = Command::new("git")
        .arg("upload-pack")
        .arg("--stateless-rpc")
        .arg(repo_path.as_ref())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let stdin = child.stdin.as_mut().ok_or(anyhow!("Failed to open stdin"))?;
    change_set.stream_to(stdin).await?;

    let output = child.wait_with_output().await?;
    Ok(output.stdout)
}

pub async fn receive_pack<P: AsRef<Path>>(repo_path: P, change_set: DataStream) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut child = Command::new("git")
        .arg("receive-pack")
        .arg("--stateless-rpc")
        .arg(repo_path.as_ref())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let stdin = child.stdin.as_mut().ok_or(anyhow!("Failed to open stdin"))?;
    change_set.stream_to(stdin).await?;

    let output = child.wait_with_output().await?;
    Ok(output.stdout)
}

pub enum GitContentType {
    #[allow(non_camel_case_types)]
    UPLOAD_PACK,
    #[allow(non_camel_case_types)]
    RECEIVE_PACK,
    ADVERTISEMENT(Cow<'static, str>)
}

impl<'h> Into<Header<'h>> for GitContentType {
    fn into(self) -> Header<'h> {
        match self {
            Self::UPLOAD_PACK => ContentType::new("application", "x-git-upload-pack-result").into(),
            Self::RECEIVE_PACK => ContentType::new("application", "x-git-receive-pack-result").into(),
            Self::ADVERTISEMENT(service) => ContentType::new("application", format!("x-git-{}-advertisement", service)).into(),
        }
    }
}
