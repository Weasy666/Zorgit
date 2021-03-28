use time::OffsetDateTime;
use crate::{Avatar, Email, Id, Url};


#[derive(Debug)]
pub struct Organisation {
    pub id: Id,
    pub name: String,
    pub avatar: Option<Avatar>,
    pub avatar_email: Option<Email>,
    /// This is the primary email. If you need all registered emails, use the function.
    pub email: Email,
    pub location: Option<String>,
    pub website: Option<Url>,
    pub description: Option<String>,

    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}
