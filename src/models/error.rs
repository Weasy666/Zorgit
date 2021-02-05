use rocket::http::Status;
use rocket::response::{self, Redirect, Response, Responder, status};
use thiserror::Error;
use anyhow;

use diesel::result::Error as DieselError;
use serde_json::{Error as SerdeJsonError, Value};
use std::io::Error as IOError;
use std::error::Error as StdError;

#[derive(Error, Debug)]
pub enum ZorgitError {
    #[error("Db Error: {0}")]
    DbError(#[from] DieselError),
    #[error("JSON Error: {0}")]
    JsonError(#[from] SerdeJsonError),
    #[error("IO Error: {0}")]
    IOError(#[from] IOError),
    #[error("Config Error: {0}")]
    ConfigError(String),
    #[error("Auth Error: {0}")]
    AuthError(String),
    #[error("VCS Error: {0}")]
    VCSError(String),
    #[error("Unknown kind of error: {0}")]
    UnknownError(String),
}

impl<'r> Responder<'r> for ZorgitError {
    fn respond_to(self, req: &rocket::Request<'_>) -> response::Result<'r> {
        //TODO: Add some logging
        match self {
            ZorgitError::DbError(_responder) => Response::build()
                                                            .status(Status::NotFound)
                                                            .finalize()
                                                            .respond_to(req),
            ZorgitError::JsonError(_responder) => Response::build()
                                                            .status(Status::InternalServerError)
                                                            .finalize()
                                                            .respond_to(req),
            ZorgitError::IOError(_responder) => Response::build()
                                                            .status(Status::InternalServerError)
                                                            .finalize()
                                                            .respond_to(req),
            ZorgitError::ConfigError(_responder) => Response::build()
                                                            .status(Status::InternalServerError)
                                                            .finalize()
                                                            .respond_to(req),
            ZorgitError::AuthError(_responder) => Response::build()
                                                            .status(Status::InternalServerError)
                                                            .finalize()
                                                            .respond_to(req),
            ZorgitError::VCSError(_responder) => Response::build()
                                                            .status(Status::InternalServerError)
                                                            .finalize()
                                                            .respond_to(req),
            ZorgitError::UnknownError(_responder) => Response::build()
                                                            .status(Status::InternalServerError)
                                                            .finalize()
                                                            .respond_to(req),
        }
    }
}

impl From<anyhow::Error> for ZorgitError {
    fn from(error: anyhow::Error) -> Self {
        if error.is::<DieselError>() {
            ZorgitError::DbError(error.downcast::<DieselError>().unwrap())
        }
        else if error.is::<SerdeJsonError>() {
            ZorgitError::JsonError(error.downcast::<SerdeJsonError>().unwrap())
        }
        else if error.is::<IOError>() {
            ZorgitError::IOError(error.downcast::<IOError>().unwrap())
        }
        else {
            ZorgitError::UnknownError(error.to_string())
        }
    }
}
