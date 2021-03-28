use time::OffsetDateTime;
use crate::{Avatar, Id, Url};


#[derive(Debug)]
pub struct Team {
    pub id: Id,
    pub name: String,
    pub avatar: Option<Avatar>,
    pub location: Option<String>,
    pub website: Option<Url>,
    pub description: Option<String>,

    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub last_seen_at: OffsetDateTime,
}
