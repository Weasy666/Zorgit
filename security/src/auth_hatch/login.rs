use rocket::{
    info, info_, log_, Rocket, Route, try_outcome,
    config::SecretKey,
    form::{Form, FromForm},
    http::{Cookie, CookieJar, SameSite, Status},
    response::Redirect,
    request::Request
};
use rocket_airlock::{Airlock, Communicator, Hatch};
use serde::Deserialize;
use crate::{AuthHatch, BasicAuth, Password, Session, SessionStore, credentials::SessionToken};


#[rocket::post("/login?<redirect_to>", data = "<login>")]
pub fn login(airlock: Airlock<AuthHatch>, redirect_to: String, login: Form<Login>) -> Result<Redirect, Status> {
    todo!()
}

#[derive(FromForm)]
pub struct Login {
    pub(crate) username: String,
    pub(crate) password: Password,
}
