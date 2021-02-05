use rocket::http::Status;
use rocket::response::{self, Redirect, Responder};
use crate::models::error::ZorgitError;
use std::ops;


pub enum Response<T> {
    Ok(T),
    Err(ZorgitError),
    Status(Status),
    Redirect(Redirect)
}

impl<'r, T: Responder<'r>> Responder<'r> for Response<T> {
    fn respond_to(self, req: &rocket::Request<'_>) -> response::Result<'r> {
        match self {
            Response::Ok(responder) => responder.respond_to(req),
            Response::Err(responder) => responder.respond_to(req),
            Response::Status(responder) => responder.respond_to(req),
            Response::Redirect(responder) => responder.respond_to(req),
        }
    }
}

impl<T> ops::Try for Response<T> {
    type Ok = Response<T>;
    type Error = Response<T>;

    #[inline]
    fn into_result(self) -> Result<<Response<T> as ops::Try>::Ok, <Response<T> as ops::Try>::Error> {
        match self {
            Response::Err(_) => Err(self),
            _ => Ok(self),
        }
    }

    #[inline]
    fn from_ok(v: <Response<T> as ops::Try>::Ok) -> Self {
        v
    }

    #[inline]
    fn from_error(v: <Response<T> as ops::Try>::Error) -> Self {
        v
    }
}

impl<T> From<anyhow::Error> for Response<T>
{
    fn from(error: anyhow::Error) -> Self {
        Response::Err(error.into())
    }
}
