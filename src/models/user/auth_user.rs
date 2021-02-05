use crate::models::{Email, Language, Password, User, UserInner, Url};
use std::path::PathBuf;
use chrono::NaiveDateTime;


pub struct AuthUser {
    pub id: i32,
    pub types: i16,
    pub username: String,
    pub full_name: Option<String>,
    pub avatar: PathBuf,
    pub avatar_email: Option<Email>,
    pub email: Email,
    pub password: Password,
    pub location: Option<String>,
    pub website: Option<Url>,
    pub description: Option<String>,
    pub language: Language,
    pub must_change_password: bool,
    pub is_email_hidden: bool,
    pub is_admin: bool,
    pub is_organisation: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub last_seen_at: NaiveDateTime,
}

impl AuthUser {
    pub fn authenticate(self, password: &Password) -> Result<User, User> {
        let verified = self.password.verify(password).unwrap();
        let user = User::Individual(UserInner {
                id: self.id,
                username: self.username,
                full_name: self.full_name,
                avatar: self.avatar,
                avatar_email: self.avatar_email,
                email: self.email,
                location: self.location,
                website: self.website,
                description: self.description,
                language: self.language,
                must_change_password: self.must_change_password,
                is_email_hidden: self.is_email_hidden,
                is_admin: self.is_admin,
                is_organisation: self.is_organisation,
                created_at: self.created_at,
                updated_at: self.updated_at,
                last_seen_at: self.last_seen_at,
            });
        if verified { Ok(user) } else { Err(user) }
    }
}

impl From<(crate::db::AuthUser, Email)> for AuthUser {
    fn from(origin: (crate::db::AuthUser, Email)) -> Self {
        AuthUser {
            id: origin.0.id,
            types: origin.0.types,
            username: origin.0.username,
            full_name: origin.0.full_name,
            avatar: origin.0.avatar,
            avatar_email: origin.0.avatar_email,
            email: origin.1,
            password: origin.0.password,
            location: origin.0.location,
            website: origin.0.website,
            description: origin.0.description,
            language: origin.0.language,
            must_change_password: origin.0.must_change_password,
            is_email_hidden: origin.0.is_email_hidden,
            is_admin: origin.0.is_admin,
            is_organisation: origin.0.is_organisation,
            created_at: origin.0.created_at,
            updated_at: origin.0.updated_at,
            last_seen_at: origin.0.last_seen_at,
        }
    }
}