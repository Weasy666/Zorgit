use std::error::Error;
use std::path::Path;
use std::process::Stdio;
use anyhow::anyhow;
use rocket::data::DataStream;
use tokio::process::Command;

mod routes;
mod service;

pub use routes::Server;
pub(crate) use service::Service;

pub async fn info_refs<P: AsRef<Path>>(repo_path: P, service: &service::Service) -> Result<Vec<u8>, Box<dyn Error>> {
    let output = Command::new("git")
        .arg(service.as_git_cmd())
        .arg("--stateless-rpc")
        .arg("--advertise-refs")
        .arg(repo_path.as_ref())
        .output()
        .await?;

    let packet = service.as_str();
    let length = packet.len() + 11;
    let payload = std::str::from_utf8(&output.stdout)?;

    Ok(format!("{:0>4x}# service={}\n{}0000", length, packet, payload).as_bytes().to_vec())
}

pub async fn upload_pack<P: AsRef<Path>>(repo_path: P, change_set: DataStream) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut child = Command::new("git")
        .arg(Service::UploadPack.as_git_cmd())
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
        .arg(Service::ReceivePack.as_git_cmd())
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
