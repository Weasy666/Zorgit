use std::str::FromStr;
use rocket::http::RawStr;
use rocket::request::{FromFormValue, FromForm, FormItems, FromParam};
use crate::models::{Notification, Sha1};
use fast_chemail;


#[derive(Clone, Debug)]
pub struct Email {
    pub id: i32,
    pub address: String,
    pub is_primary: bool,
    pub activated_at: Option<chrono::NaiveDateTime>,
    pub token: Option<Sha1>,
    pub token_created_at: Option<chrono::NaiveDateTime>,
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
            token: None,
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


pub struct UpdateEmail {
    pub id: i32,
    pub address: String,
    pub is_primary: bool,
    pub notification: Notification,
}
pub type NewEmail = UpdateEmail;
pub type DeleteEmail = UpdateEmail;

impl<'f> FromForm<'f> for UpdateEmail {
    type Error = anyhow::Error;

    fn from_form(items: &mut FormItems<'f>, _strict: bool) -> Result<UpdateEmail, Self::Error> {
        let mut id = -1;
        let mut address = std::default::Default::default();
        let mut is_primary = false;
        let mut notification = Notification::Disabled;

        for form_item in items {
            let (key, value) = form_item.key_value_decoded();
            match key.as_str() {
                "id" => id = value.parse::<i32>().expect("Unable to parse id as i32"),
                "address" => address = fast_chemail::parse_email(&value).map(|_| value.to_string())?,
                "is_primary" => is_primary = value == "true",
                "notification" => notification = Notification::from(value),
                _ => (),
            }
        }

        if address.is_empty() {
            return Err(anyhow!("Empty address in FromForm for UpdateEmail."));
        }

        Ok(UpdateEmail {
            id,
            address,
            is_primary,
            notification,
        })
    }
}