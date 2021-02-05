use std::path::PathBuf;
use blockies::Ethereum;
use uuid::*;
use crate::models::{Config};
use walkdir::WalkDir;

pub mod crypto;
pub mod humanize;
pub mod render;

/// Generates a blockies default avatar from the given value and saves it in the configured
/// avatars folder. The return value is the relative path to the generated image.
pub fn create_default_avatar(value: &str) -> Result<PathBuf, String> {
    let blockies = Ethereum::default();
    let uuid = Uuid::new_v5(&Uuid::NAMESPACE_URL, value.as_bytes()).to_simple();
    let mut path = Config::global().avatars_dir();
    path.push(format!("{:x}.png", uuid));

    let png_file = std::fs::File::create(&path).map_err(|e| e.to_string())?;
    blockies.create_icon(png_file, uuid.to_string().as_bytes()).map_err(|_e| "Something went wrong with pixelate".to_string())?;

    Ok(PathBuf::from(format!("/avatars/{:x}.png", uuid)))
}

pub trait IntoOption {
    type Output;
    fn into_option(self) -> Option<Self::Output>;
}

impl<T> IntoOption for Vec<T> {
    type Output = Vec<T>;
    fn into_option(self) -> Option<Self::Output> {
        if self.is_empty() {
            None
        }
        else {
            Some(self)
        }
    }
}


pub fn calc_size(path: &PathBuf) -> u64 {
    if path.is_file() {
        return path.metadata().map(|m| m.len()).unwrap_or(0);
    }

    WalkDir::new(path)
        .into_iter()
        .filter_map(|entry| {
            if entry.is_err() {
                return None;
            }
            let entry = entry.unwrap();
            let path = entry.path();

            let metadata = path.symlink_metadata();
            if metadata.is_err() {
                return None;
            }
            Some(metadata.unwrap().len())
        })
        .sum()
}