use std::str::FromStr;
use fast_chemail;
use rocket::http::RawStr;
use rocket::request::{FromForm, FromFormValue, FormItems, FromParam};
use time::OffsetDateTime;
use crate::{Notification, Sha1};


#[derive(Clone, Debug)]
pub struct Email {
    pub id: i32,
    pub address: String,
    pub is_primary: bool,
    pub activated_at: Option<OffsetDateTime>,
    //pub token: Option<Sha1>,
    pub token_created_at: Option<OffsetDateTime>,
    pub notification: Notification,
}

impl Email {
    pub fn as_primary(mut self, enable: bool) -> Email {
        self.is_primary = enable;
        self
    }

    pub fn is_activated(&self) -> bool {
        self.activated_at.is_some()
    }
}

impl<'a> FromFormValue<'a> for Email {
    type Error = &'a RawStr;

    fn from_form_value(form_value: &'a RawStr) -> Result<Email, Self::Error> {
        form_value.parse::<Email>()
            .map_err(|_| form_value)
    }
}

impl<'a> FromParam<'a> for Email {
    type Error = &'a RawStr;

    fn from_param(param: &'a RawStr) -> Result<Email, Self::Error> {
        param.parse::<Email>()
            .map_err(|_| param)
    }
}

impl FromStr for Email {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Email, Self::Err> {
        fast_chemail::parse_email(s)?;
        Ok(Email {
            id: -1,
            address: s.to_string(),
            is_primary: false,
            activated_at: None,
            //token: None,
            token_created_at: None,
            notification: Notification::Enabled,
        })
    }
}

impl std::fmt::Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.address, f)
    }
}
