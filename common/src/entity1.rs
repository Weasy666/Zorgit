use rocket::{
    error, info, info_, log_, warn,
    request::{FromRequest, Outcome, Request}
};
use rocket_contrib::databases::{Connection, Poolable};
use rocket_airlock::Airlock;
use std::{marker::PhantomData, ops::{Deref, DerefMut}};
use time::OffsetDateTime;
use unic_langid::LanguageIdentifier;
use zorgit_security::{AuthHatch, BasicAuth};
use crate::{Avatar, Email, Url};


#[derive(Debug, Clone)]
pub enum Entity<A> {
    Organisation(EntityInner<A>),
    Team(EntityInner<A>),
    Unknown(EntityInner<A>),
    User(EntityInner<A>),
}

impl Deref for Entity<AuthHatch> {
    type Target = EntityInner<AuthHatch>;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Organisation(user) => user,
            Self::Team(user) => user,
            Self::Unknown(user) => user,
            Self::User(user) => user,
        }
    }
}

impl DerefMut for Entity<AuthHatch> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::Organisation(user) => user,
            Self::Team(user) => user,
            Self::Unknown(user) => user,
            Self::User(user) => user,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EntityInner<A> {
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
    _marker: PhantomData<A>,
}

impl Entity<AuthHatch> {
    pub fn unknown(email: &str) -> Entity<AuthHatch> {
        Entity::Unknown(EntityInner {
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
            _marker: PhantomData,
        })
    }
}

// #[rocket::async_trait]
// impl<'a,'r, H: Hatch> FromRequest<'a, 'r> for Entity<Airlock<H>> {
//     type Error = Box<dyn std::error::Error>;

//     async fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
//         let conn = match request.guard::<Airlock<H>>().await {
//             Outcome::Success(conn) => conn,
//             Outcome::Failure((status, error)) => return Outcome::Failure((status, "TODO".into())),
//             Outcome::Forward(f) => return Outcome::Forward(f),
//         };

//         Entity::from
//     }
// }

impl PartialEq for Entity<AuthHatch> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.username == other.username
    }
}

#[rocket::async_trait]
impl<'a, 'r> FromRequest<'a, 'r> for Entity<AuthHatch> {
    type Error = Box<dyn std::error::Error>;

    async fn from_request(request: &'a Request<'r>) -> rocket::request::Outcome<Entity<AuthHatch>, Self::Error> {
        let hatch = match request.guard::<Airlock<AuthHatch>>().await {
            Outcome::Success(hatch) => hatch,
            Outcome::Failure((status, error)) => return Outcome::Failure((status, "TODO".into())),
            Outcome::Forward(f) => return Outcome::Forward(f),
        }.hatch;
        let mut entity;

        let conn = hatch.connection(request).await.unwrap();
        entity = match hatch.basic_auth(request) {
            Ok(auth) => {
                info_!("An Entity is trying to gain access with the Authorization Header: {} {}:[redacted]", &auth.r#type, &auth.username);
                //TODO: actually authenticate the user and retriev him from database
                Ok(())
            }
            Err(e) => {
                warn!("Malformed Authorization Header: {}", e);
                Err(e)
            }
        };

        entity = match hatch.session_auth(request).await {
            Ok(session) => {
                info_!("An Entity is trying to gain access with the Session: {} {}", &session.user_id, &session.token);
                //TODO: actually authenticate the user and retrieve him from database
                Ok(())
            }
            Err(e) => {
                warn!("Malformed Authorization Header: {}", e);
                Err(e)
            }
        };

        // else {
        //     // check if sessionID is valid and get user data from DB
        //     user = request.cookies()
        //         .get_private(&Self::get_cookie_key())
        //         .ok_or("No Session found!".to_string())
        //         .and_then(|sid| Session::validate(&conn, sid.value()).map_err(|_| "No valid session found!".to_string()))
        //         .and_then(|session|
        //             User::by_id(&conn, session.user.id)
        //             .map_err(|_| "User not found!".to_string())
        //         );
        // }

        match entity {
            Ok(entity) => Outcome::Success(entity),
            _ => Outcome::Forward(()),
        }
    }
}
