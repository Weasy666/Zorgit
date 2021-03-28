use unic_langid::LanguageIdentifier;
use crate::{Avatar, Email};


#[derive(Debug)]
pub struct Unknown {
    pub username: String,
    pub full_name: Option<String>,
    pub avatar: Option<Avatar>,
    /// This is the primary email. If you need all registered emails, use the function.
    pub email: Email,
    pub language: LanguageIdentifier,
}
