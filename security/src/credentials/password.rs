use std::error::Error;
use argon2;
use rand::{RngCore, SeedableRng};
use rand_chacha::ChaChaRng;
use rocket::data::ToByteUnit;
use rocket::form::{self, DataField, FromFormField, ValueField};

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
    /// Parses the given String as password and enforces global password rules on parsing.
    /// It is assumed that every use of `.parse::<Password>()` or `Password::from_str()`
    /// is used on a String representation of a plain password.
    pub fn from_plain_str<S: AsRef<str>>(plain_pwd: S) -> Result<Password, Box<dyn Error>> {
        let plain_pwd = plain_pwd.as_ref();
        // TODO: maybe use a Regex or something else which can be specified in the config file
        if plain_pwd.len() < 8 {
            Err("Not a valid password as per global password rules: Password too short.")?
        }
        else {
            Ok(Password::Plain(plain_pwd.to_string()))
        }
    }

    // /// Parses the given String as password and enforces global password rules on parsing.
    // /// It is assumed that every use of `.parse::<Password>()` or `Password::from_str()`
    // /// is used on a String representation of a plain password.
    // pub fn from_hash_str<S: AsRef<str>>(plain_pwd: S) -> Result<Password, Box<dyn Error>> {
    //     todo!()
    // }

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
    pub fn verify(&self, plain_passwd: &Password) -> Result<bool, Box<dyn Error>> {
        match plain_passwd {
            Password::Hashed(_) => return Err("You need to provide a plain password against which the hashed password can be verified.")?,
            Password::Plain(plain) => match self {
                Password::Hashed(hash) => {
                    let verfied = argon2::verify_encoded(hash, plain.as_bytes())?;
                    Ok(verfied)
                }
                Password::Plain(_) => Err("You tried to verify a password on a None value. Please load an already hashed password form the database and try again.")?,
            }
        }
    }

    pub fn password(&self) -> Result<String, Box<dyn Error>> {
        match self {
            Password::Hashed(hash) => {
                Ok(hash.splitn(5, '$')
                    .nth(4)
                    .map(ToString::to_string)
                    .ok_or("Seems to not be a valid argon2 hash.")?)
            }
            Password::Plain(_) => Err("You tried to get an unhashed password, this is not allowed.")?,
        }
    }

    pub fn salt(&self) -> Result<String, Box<dyn Error>> {
        match self {
            Password::Hashed(hash) => {
                let mut hash = hash.splitn(5, '$');
                hash.next().ok_or("Seems to not be a valid argon2 hash.")?;
                let salt = format!("${}${}${}",
                    hash.next().ok_or("Seems to not be a valid argon2 hash.")?,
                    hash.next().ok_or("Seems to not be a valid argon2 hash.")?,
                    hash.next().ok_or("Seems to not be a valid argon2 hash.")?
                );
                Ok(salt)
            }
            Password::Plain(_) => Err("You tried to get the salt of an unhashed password, this is not possible.")?,
        }
    }
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for Password {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        match Password::from_plain_str(field.value) {
            Ok(pwd) => Ok(pwd),
            Err(e) => Err(form::Error::validation(e.to_string()))?,
        }
    }

    async fn from_data(field: DataField<'r, '_>) -> form::Result<'r, Self> {
        // Retrieve the configured data limit or use `256KiB` as default.
        let limit = field.request.limits()
            .get("password")
            .unwrap_or(256.kibibytes());

        // Read the capped data stream, returning a limit error as needed.
        let bytes = field.data.open(limit).into_bytes().await?;
        if !bytes.is_complete() {
            Err((None, Some(limit)))?;
        }

        // Store the bytes in request-local cache.
        let bytes = bytes.into_inner();
        let bytes = rocket::request::local_cache!(field.request, bytes);

        // Try to parse the name as UTF-8 or return an error if it fails.
        let pwd = std::str::from_utf8(bytes)?;
        match Password::from_plain_str(pwd) {
            Ok(pwd) => Ok(pwd),
            Err(e) => Err(form::Error::validation(e.to_string()))?,
        }
    }
}

// impl<'a> FromParam<'a> for Password {
//     type Error = Box<dyn Error>;

//     fn from_param(param: &'a RawStr) -> Result<Password, Box<dyn Error>> {
//         Ok(param.parse::<Password>()?)
//     }
// }

impl PartialEq for Password {
    fn eq(&self, other: &Self) -> bool {
        match (&self, &other) {
            (&Password::Plain(s), &Password::Plain(o)) => s == o,
            (&Password::Hashed(s), &Password::Hashed(o)) => s == o,
            _ => false,
        }
    }
}

impl std::fmt::Display for Password {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Password::Hashed(hash) => write!(f, "{}", hash),
            Password::Plain(_) => panic!("It is not allowed to convert or display a plain password. You must hash it first."),
        }
    }
}
