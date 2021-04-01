use std::str::FromStr;
use fast_chemail;
use rocket::{
    data::ToByteUnit,
    form::{self, DataField, FromFormField, ValueField},
    http::RawStr,
    request::FromParam
};
use time::OffsetDateTime;
use crate::Notification;


#[derive(Debug)]
pub struct Email {
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

#[rocket::async_trait]
impl<'r> FromFormField<'r> for Email {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        Ok(field.value.parse::<Email>()
            .map_err(|e| form::Error::validation(format!("{}", e)))?)
    }

    async fn from_data(field: DataField<'r, '_>) -> form::Result<'r, Self> {
        // Retrieve the configured data limit or use `256KiB` as default.
        let limit = field.request.limits()
            .get("email")
            .unwrap_or(256.kibibytes());

        // Read the capped data stream, returning a limit error as needed.
        let bytes = field.data.open(limit).into_bytes().await?;
        if !bytes.is_complete() {
            Err((None, Some(limit)))?;
        }

        // Store the bytes in request-local cache and split at ':'.
        let bytes = bytes.into_inner();
        let bytes = rocket::request::local_cache!(field.request, bytes);
        // Try to parse the name as UTF-8 or return an error if it fails.
        let email = std::str::from_utf8(bytes)?;
        Ok(email.parse::<Email>()
            .map_err(|e| form::Error::validation(format!("{}", e)))?)
    }
}

impl<'r> FromParam<'r> for Email {
    type Error = &'r str;

    fn from_param(param: &'r str) -> Result<Email, Self::Error> {
        RawStr::new(param).parse::<Email>()
            .map_err(|_| param)
    }
}

impl FromStr for Email {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Email, Self::Err> {
        fast_chemail::parse_email(s)?;
        Ok(Email {
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
