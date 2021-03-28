use std::ops::{Deref, DerefMut};
use std::str::FromStr;
use anyhow::{anyhow, Result};
use rocket::{
    data::ToByteUnit,
    form::{self, DataField, FromForm, FromFormField, ValueField},
    request::FromParam
};


#[derive(Clone, Debug)]
/// A SHA-1 hash.
pub struct Sha1(String);

impl Sha1 {
    /// Generates a new Sha1 hash for the given value.
    pub fn new<V: AsRef<[u8]>>(value: V) -> Sha1 {
        let mut sha1 = sha1::Sha1::new();
        sha1.update(value.as_ref());
        Sha1(sha1.digest().to_string())
    }
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for Sha1 {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        Ok(field.value.parse::<Sha1>()
            .map_err(|e| form::Error::validation(format!("{}", e)))?)
    }

    async fn from_data(field: DataField<'r, '_>) -> form::Result<'r, Self> {
        // Retrieve the configured data limit or use `256KiB` as default.
        let limit = field.request.limits()
            .get("sha1")
            .unwrap_or(20.bytes());

        // Read the capped data stream, returning a limit error as needed.
        let bytes = field.data.open(limit).into_bytes().await?;
        if !bytes.is_complete() {
            Err((None, Some(limit)))?;
        }

        // Store the bytes in request-local cache and split at ':'.
        let bytes = bytes.into_inner();
        let bytes = rocket::request::local_cache!(field.request, bytes);
        // Try to parse the name as UTF-8 or return an error if it fails.
        let hash = std::str::from_utf8(bytes)?;
        Ok(hash.parse::<Sha1>()
            .map_err(|e| form::Error::validation(format!("{}", e)))?)
    }
}

impl<'r> FromParam<'r> for Sha1 {
    type Error = &'r str;

    fn from_param(param: &'r str) -> Result<Sha1, Self::Error> {
        param.parse::<Sha1>()
            .map_err(|_| param)
    }
}

impl FromStr for Sha1 {
    type Err = anyhow::Error;

    /// Parses the given String and checks if it is a valid Sha1 value.
    /// Regex: ^[a-fA-F0-9]{40}$
    fn from_str(s: &str) -> Result<Sha1, Self::Err> {
        if s.len() != 40 {
            return Err(anyhow!("Not a valid SHA-1 value!"));
        }

        let mut is_sha1 = true;
        for c in s.as_bytes() {
            match *c {
                b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F' => (),
                _ => is_sha1 = false,
            }
        }

        if is_sha1 {
            Ok(Sha1(s.to_string()))
        }
        else {
            Err(anyhow!("Not a valid SHA-1 value!"))
        }
    }
}

impl std::fmt::Display for Sha1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl Deref for Sha1 {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Sha1 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
