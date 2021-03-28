use std::borrow::Borrow;
use std::str::FromStr;
use rocket::{
    data::ToByteUnit,
    form::{self, DataField, FromForm, FromFormField, ValueField},
    request::FromParam
};


#[derive(Clone, Debug, PartialEq)]
pub enum Notification {
    Disabled,
    Enabled,
    OnMentions,
}

impl Notification {
    pub fn to_i16(&self) -> i16 {
        self.into()
    }
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for Notification {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        Ok(field.value.parse::<Notification>()
            .map_err(|e| form::Error::validation(format!("{}", e)))?)
    }

    async fn from_data(field: DataField<'r, '_>) -> form::Result<'r, Self> {
        // Retrieve the configured data limit or use `256KiB` as default.
        let limit = field.request.limits()
            .get("notification")
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
        let notification = std::str::from_utf8(bytes)?;
        Ok(notification.parse::<Notification>()
            .map_err(|e| form::Error::validation(format!("{}", e)))?)
    }
}

impl<'r> FromParam<'r> for Notification {
    type Error = &'r str;

    fn from_param(param: &'r str) -> Result<Notification, Self::Error> {
        param.parse::<Notification>()
            .map_err(|_| param)
    }
}

impl FromStr for Notification {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Notification, Self::Err> {
        match s.to_lowercase().as_ref() {
            "disabled" => Ok(Notification::Disabled),
            "enabled" => Ok(Notification::Enabled),
            "onmentions" => Ok(Notification::OnMentions),
            _ => {
                let number = s.parse::<i16>()?;
                Ok(Notification::from(number))
            },
        }
    }
}

impl From<i16> for Notification {
    fn from(number: i16) -> Self {
        match number {
            0 => Notification::Disabled,
            1 => Notification::Enabled,
            2 => Notification::OnMentions,
            _ => panic!("Unknown type of Notification"),
        }
    }
}

impl Into<i16> for &Notification {
    fn into(self) -> i16 {
        match self {
            Notification::Disabled => 0,
            Notification::Enabled => 1,
            Notification::OnMentions => 2,
        }
    }
}

impl Into<i16> for Notification {
    fn into(self) -> i16 {
        self.borrow().into()
    }
}

impl<'f> From<&'f str> for Notification {
    fn from(value: &'f str) -> Self {
        match value.to_lowercase().as_ref() {
            "disabled" => Notification::Disabled,
            "enabled" => Notification::Enabled,
            "onmentions" => Notification::OnMentions,
            _ => {
                let number = value.parse::<i16>().expect("Unknown type of Notification");
                Notification::from(number)
            },
        }
    }
}

impl From<String> for Notification {
    fn from(value: String) -> Self {
        Notification::from(&value)
    }
}

impl From<&String> for Notification {
    fn from(value: &String) -> Self {
        let val: &str = value.borrow();
        Notification::from(val)
    }
}
