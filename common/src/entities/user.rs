use rocket::{Request, request::{FromRequest, Outcome}};
use rocket_airlock::Airlock;
use time::OffsetDateTime;
use unic_langid::LanguageIdentifier;
use zorgit_security::AuthHatch;
use crate::{Avatar, Email, Id, Url};


#[derive(Debug)]
pub struct User {
    pub id: Id,
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

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = Box<dyn std::error::Error>;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let hatch = match request.guard::<Airlock<AuthHatch>>().await {
            Outcome::Success(hatch) => hatch,
            Outcome::Failure((status, error)) => return Outcome::Failure((status, "TODO".into())),
            Outcome::Forward(f) => return Outcome::Forward(f),
        }.hatch;


        todo!()
    }
}
