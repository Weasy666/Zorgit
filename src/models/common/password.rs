use std::str::FromStr;
use std::fmt;
use rocket::http::RawStr;
use rocket::request::{FromFormValue, FromParam};
use argon2;
use rand::{RngCore, SeedableRng};
use rand_chacha::ChaChaRng;
use anyhow::Result;


/// An enum for password handling. At creation the given password is validated and immediately
/// afterwards the plain text password is converted to a hash and stored internally as a hashed String.  
/// Be aware that the hash is created with a random salt, which means that continued creation of
/// a Password struct from the same plain text password will lead to a different hash.
#[derive(Clone, Debug)]
pub enum Password {
    Plain(String),
    Hashed(String),
}

impl Password {
    /// Consumes current plain text password and creates a new one, hashed with argon2.
    pub fn hash(self) -> Password {
        match self {
            Password::Plain(plain) => {
                let mut rnd = ChaChaRng::from_entropy();
                let mut salt = [0u8; 32];
                rnd.fill_bytes(&mut salt);
                
                let mut config = argon2::Config::default();
                config.variant = argon2::Variant::Argon2id;
                let hashed = argon2::hash_encoded(plain.as_bytes(), &salt, &config).unwrap();
                Password::Hashed(hashed)
            }
            Password::Hashed(_) => self,
        }
    }
    
    /// 
    pub fn verify(&self, plain_passwd: &Password) -> Result<bool> {
        match plain_passwd {
            Password::Hashed(_) => return Err(anyhow!("You need to provide a plain password against which the hashed password can be verified.")),
            Password::Plain(plain) => match self {
                Password::Hashed(hash) => {
                    let verfied = argon2::verify_encoded(hash, plain.as_bytes())?;
                    Ok(verfied)
                }
                Password::Plain(_) => Err(anyhow!("You tried to verify a password on a None value. Please load an already hashed password form the database and verify again.")),
            }
        }
    }

    pub fn password(&self) -> Result<String> {
        match self {
            Password::Hashed(hash) => {
                hash.splitn(5, '$')
                    .nth(4)
                    .map(ToString::to_string)
                    .ok_or(anyhow!("Seems to not be a valid argon2 hash."))
            }
            Password::Plain(_) => Err(anyhow!("You tried to get an unhashed password, this is not allowed.")),
        }
    }

    pub fn salt(&self) -> Result<String> {
        match self {
            Password::Hashed(hash) => {
                let mut hash = hash.splitn(5, '$');
                hash.next().ok_or(anyhow!("Seems to not be a valid argon2 hash."))?;
                let salt = format!("${}${}${}",
                    hash.next().ok_or(anyhow!("Seems to not be a valid argon2 hash."))?,
                    hash.next().ok_or(anyhow!("Seems to not be a valid argon2 hash."))?,
                    hash.next().ok_or(anyhow!("Seems to not be a valid argon2 hash."))?
                );
                Ok(salt)
            }
            Password::Plain(_) => Err(anyhow!("You tried to get the salt of an unhashed password, this is not possible.")),
        }
    }
}

impl<'a> FromFormValue<'a> for Password {
    type Error = &'a RawStr;

    fn from_form_value(form_value: &'a RawStr) -> Result<Password, Self::Error> {
        form_value.parse::<Password>()
            .map_err(|_| form_value)
    }
}

impl<'a> FromParam<'a> for Password {
    type Error = &'a RawStr;

    fn from_param(param: &'a RawStr) -> Result<Password, Self::Error> {
        param.parse::<Password>()
            .map_err(|_| param)
    }
}

impl PartialEq for Password {
    fn eq(&self, other: &Self) -> bool {
        match (&self, &other) {
            (&Password::Plain(s), &Password::Plain(o)) => s == o,
            (&Password::Hashed(s), &Password::Hashed(o)) => s == o,
            _ => false,
        }
    }
}

impl fmt::Display for Password {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Password::Hashed(hash) => write!(f, "{}", hash),
            Password::Plain(_) => panic!("It is not allowed to convert or display a plain password. You must hash it first."),
        }
    }
}

impl FromStr for Password {
    type Err = anyhow::Error;

    /// Parses the given String as password and enforces global password rules on parsing.  
    /// It is assumed that every use of `.parse::<Password>()` or `Password::from_str()`
    /// is used on a String representation of a plain password.
    fn from_str(s: &str) -> Result<Password, Self::Err> {
        // TODO: maybe use a Regex or something else which can be specified in the config file
        if s.len() < 8 {
            Err(anyhow!("Not a valid password as per global password rules: Password too short."))
        }
        else {
            Ok(Password::Plain(s.to_string()))
        }
    }
}