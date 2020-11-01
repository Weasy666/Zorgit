use std::fs::File;
use std::path::{Path, PathBuf};
use anyhow::anyhow;
use blockies::Ethereum;
use uuid::Uuid;
use crate::Result;

#[derive(Debug, Clone)]
pub struct Avatar {
    path: PathBuf,
}

impl Avatar {
    /// Generates a blockies default avatar from the given value and saves it in the
    /// specified folder. The return value is the relative path to the generated image.
    pub fn default_from<P: AsRef<Path>>(value: &str, path: P) -> Result<Avatar> {
        let blockies = Ethereum::default();
        let uuid = Uuid::new_v5(&Uuid::NAMESPACE_URL, value.as_bytes()).to_simple();
        let path = path.as_ref().join(format!("{:x}.png", uuid));

        let png_file = File::create(&path)?;
        blockies.create_icon(png_file, uuid.to_string().as_bytes())
            .map_err(|e| anyhow!("Something went wrong with pixelate: {:?}", e))?;

        Ok(Avatar{ path })
    }
}
