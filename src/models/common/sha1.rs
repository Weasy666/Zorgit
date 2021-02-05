use std::ops::{Deref, DerefMut};
use std::str::FromStr;
use rocket::http::RawStr;
use rocket::request::{FromFormValue, FromParam};
use anyhow::Result;


#[derive(Clone, Debug)]
pub struct Sha1(String);

impl Sha1 {
    /// Generates a new Sha1 hash for the given value.
    pub fn new<V: AsRef<[u8]>>(value: V) -> Sha1 {
        let mut sha1 = sha1::Sha1::new();
        sha1.update(value.as_ref());
        Sha1(sha1.digest().to_string())
    }
}

impl<'a> FromFormValue<'a> for Sha1 {
    type Error = &'a RawStr;

    fn from_form_value(form_value: &'a RawStr) -> Result<Sha1, Self::Error> {
        form_value.parse::<Sha1>()
            .map_err(|_| form_value)
    }
}

impl<'a> FromParam<'a> for Sha1 {
    type Error = &'a RawStr;

    fn from_param(param: &'a RawStr) -> Result<Sha1, Self::Error> {
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