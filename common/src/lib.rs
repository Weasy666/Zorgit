mod avatar;
mod dotfile;
mod email;
mod entity;
mod notification;
mod password;
mod sha1;
mod url;

pub use avatar::Avatar;
pub use dotfile::DotFile;
pub use email::Email;
pub use entity::Entity;
pub use notification::Notification;
pub use password::Password;
pub use crate::sha1::Sha1;
pub use crate::url::Url;


use std::error::Error;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

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
