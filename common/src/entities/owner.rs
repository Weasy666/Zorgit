use rocket::{Request, request::{FromParam, FromRequest, Outcome}};
use rocket_airlock::Airlock;
use zorgit_security::AuthHatch;
use crate::entities::{Organisation, User};


#[derive(Debug)]
pub enum Owner {
    User(User),
    Organisation(Organisation),
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Owner {
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

impl<'r> FromParam<'r> for Owner {
    type Error = Box<dyn std::error::Error>;

    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        todo!()
    }
}
