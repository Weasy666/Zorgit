use std::ops::{Deref, DerefMut};
use time::OffsetDateTime;
use unic_langid::LanguageIdentifier;
use crate::{Avatar, Email, Url};


#[derive(Debug, Clone)]
pub enum Entity {
    Organisation(EntityInner),
    Team(EntityInner),
    User(EntityInner),
}

impl Deref for Entity {
    type Target = EntityInner;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Organisation(user) => user,
            Self::Team(user) => user,
            Self::User(user) => user,
        }
    }
}

impl DerefMut for Entity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::Organisation(user) => user,
            Self::Team(user) => user,
            Self::User(user) => user,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EntityInner {
    pub id: i32,
    pub username: String,
    pub full_name: Option<String>,
    pub avatar: Option<Avatar>,
    pub avatar_email: Option<Email>,
    /// This is the primary email. If you need all registered emails, use the function.
    pub email: Email,
    pub location: Option<String>,
    pub website: Option<Url>,
    pub description: Option<String>,
    pub language: LanguageIdentifier,

    pub must_change_password: bool,
    pub is_email_hidden: bool,
    pub is_admin: bool,

    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub last_seen_at: OffsetDateTime,
}

impl Entity {
    pub fn unknown(email: &str) -> Entity {
        Entity::User(EntityInner {
            id: -1,
            username: email.to_string(),
            full_name: None,
            avatar: None,
            avatar_email: None,
            email: email.parse::<Email>().expect(&format!("Not a valid email address: {}", email)),
            location: None,
            website: None,
            description: None,
            language: "en-EN".parse().expect("Parse 'en-EN' into 'LanguageIdentifier'"),

            must_change_password: false,
            is_email_hidden: false,
            is_admin: false,

            created_at: OffsetDateTime::from_unix_timestamp(0),
            updated_at: OffsetDateTime::from_unix_timestamp(0),
            last_seen_at: OffsetDateTime::from_unix_timestamp(0),
        })
    }
}

impl PartialEq for Entity {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
