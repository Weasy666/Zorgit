use std::str::FromStr;
use data_encoding::BASE64;

use crate::Password;

pub struct BasicAuth {
    pub r#type: String,
    pub username: String,
    pub password: Password,
}

impl FromStr for BasicAuth {
    type Err = Box<dyn std::error::Error>;

    fn from_str(auth_header: &str) -> Result<Self, Self::Err> {
        let mut header = auth_header.splitn(2, " ");
        let r#type = header.next()
            .map(ToString::to_string)
            .ok_or("Authorization Header is missing the type information")?;

        let credentials = header.next()
            .ok_or("Authorization Header is missing the credentials")?;
        let credentials = String::from_utf8(BASE64.decode(credentials.as_bytes())?)?;
        let mut credentials = credentials.splitn(2, ":");

        let username = credentials.next()
            .map(ToString::to_string)
            .ok_or("Missing credentials")?;

        let password = credentials.next()
            .ok_or("Missing credentials".into())
            .and_then(|p| Password::from_plain_str(p))?;

        Ok(BasicAuth {
            r#type,
            username,
            password,
        })
    }
}
