use std::error::Error;
use std::str::FromStr;
use rocket::{
    info, info_, log_, Rocket, Route, try_outcome,
    config::SecretKey,
    http::{Cookie, CookieJar, SameSite, Status},
    response::Redirect,
    request::Request
};
use rocket_airlock::{Airlock, Communicator, Hatch};
use serde::Deserialize;
use crate::{BasicAuth, Session, credentials::SessionToken};

pub struct AuthHatch {
    config: Config,
    comm: crate::SessionStore,
}

impl<'a> AuthHatch {
    pub fn session_key(&self) -> &str {
        &self.config.session_key
    }

    pub fn basic_auth<'r>(&self, request: &Request<'r>) -> Result<BasicAuth, Box<dyn Error>> {
        let header = request.headers()
            .get_one("Authorization")
            .ok_or("No Authorization Header found!")?;

        header.parse::<BasicAuth>()
    }

    pub async fn session_auth<'r>(&self, request: &Request<'r>) -> Result<Session, Box<dyn Error>> {
        let session_cookie = request.cookies().get_private_pending(self.session_key())
            .ok_or("No Session found!")?
            .value()
            .to_string();

        // username+0xff+plain_sessiontoken
        let mut username_token = session_cookie.as_bytes()
            .splitn(2 ,|&b| b == 0xff);
        let username = String::from_utf8_lossy(username_token.next().unwrap());
        let plain_token = SessionToken::from_bytes(username_token.next().unwrap())?;

        if let Some(session) = self.comm.session_by_token("user_id", plain_token)? {
            Ok(session)
        } else {
            Err("No Session found!")?
        }
    }

    pub async fn connection<'r>(&self, request: &Request<'r>) -> Result<crate::SessionStore, Box<dyn std::error::Error>> {
        let conn = request.guard::<crate::SessionStore>().await.unwrap();
        Ok(conn)
    }

    // pub fn authenticate_by_name(&self, username: &str) -> bool {
    //     let mut prefix = username.as_bytes().to_vec();
    //     prefix.push(0xff);
    //     self.comm.scan_prefix(prefix)
    //         .find(|(a,b)| {})
    // }

    pub fn is_session_expired(&self, session: Session) -> Result<bool, Box<dyn std::error::Error>> {
        self.comm.is_expired_session(session)
    }
}

#[rocket::async_trait]
impl Hatch for AuthHatch {
    type Comm = crate::SessionStore;

    fn name() -> &'static str { "Auth" }

    fn routes() -> Vec<Route> {
        rocket::routes![login]
    }

    async fn from(rocket: &Rocket) -> Result<AuthHatch, Box<dyn std::error::Error>> {
        let name = AuthHatch::name().replace(" ", "").to_lowercase();
        let config = rocket.figment().extract_inner::<Config>(&format!("airlock.{}", name))?;
        Ok(AuthHatch {
            config,
            comm: Communicator::from(rocket).await?
        })
    }

    fn comm(&self) -> &Self::Comm { &self.comm }
}

pub type Days = u64;

#[derive(Debug, Deserialize)]
struct Config {
    /// Secret key that is used as seed
    pub secret_key: SecretKey,
    /// Name with which the session value will be added to a cookie
    pub session_key: String,
    /// Duration of a login session in days
    pub session_duration: Days,
}

#[rocket::post("/login?<redirect_to>", data = "<login>")]
pub fn login(airlock: Airlock<AuthHatch>, redirect_to: String, login: String) -> Result<Redirect, Status> {
    todo!()
}
