use std::error::Error;
use argon2;
use rand::{RngCore, SeedableRng};
use rand_chacha::ChaChaRng;
use rocket::http::{Cookie, SameSite};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;


#[derive(Serialize, Deserialize)]
pub enum SessionToken {
    Plain(String),
    Hashed(String),
}

impl SessionToken {
    pub fn hash(self) -> SessionToken {
        match self {
            SessionToken::Plain(plain) => {
                let mut rnd = ChaChaRng::from_entropy();
                let mut salt = [0u8; 32];
                rnd.fill_bytes(&mut salt);

                let mut config = argon2::Config::default();
                config.variant = argon2::Variant::Argon2id;
                let hashed = argon2::hash_encoded(plain.as_bytes(), &salt, &config).unwrap();
                SessionToken::Hashed(hashed)
            }
            SessionToken::Hashed(_) => self,
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<SessionToken, Box<dyn Error>> {
        let plain = String::from_utf8(bytes.to_vec())?;
        Ok(SessionToken::Plain(plain))
    }
}

impl PartialEq for SessionToken {
    fn eq(&self, other: &Self) -> bool {
        match (&self, &other) {
            (&SessionToken::Plain(s), &SessionToken::Plain(o)) => s == o,
            (&SessionToken::Hashed(s), &SessionToken::Hashed(o)) => s == o,
            _ => false,
        }
    }
}

impl std::fmt::Display for SessionToken {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SessionToken::Hashed(hash) => write!(f, "{}", hash),
            SessionToken::Plain(_) => panic!("It is not allowed to convert or display a plain session token. You must hash it first."),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Session {
        pub user_id: String,
        pub created_at: OffsetDateTime,
        pub expires_at: OffsetDateTime,
        pub token: SessionToken,
}

impl Session {
    pub fn to_cookie(self, session_key: &str) -> Result<Cookie<'_>, Box<dyn Error>> {
        match self.token {
            SessionToken::Plain(token) => {
                Ok(Cookie::build(session_key, token)
                    .same_site(SameSite::Strict)
                    .http_only(true)
                    .finish())
             }
             SessionToken::Hashed(_) => Err("You tried to create a cookie from a session with a hashed token, this is not allowed.")?,
        }
    }

    pub fn validate(&self, plain_token: &SessionToken) -> Result<bool, Box<dyn Error>> {
        match plain_token {
            SessionToken::Hashed(_) => return Err("You need to provide a plain token against which the hashed token can be verified.")?,
            SessionToken::Plain(plain) => match &self.token {
                SessionToken::Hashed(hash) => {
                    let verfied = argon2::verify_encoded(hash, plain.as_bytes())?;
                    Ok(verfied)
                }
                SessionToken::Plain(_) => Err("You tried to verify a session token on a None value. Please get valid session from the store and try again.")?,
            }
        }
    }
}

impl PartialEq for Session {
    fn eq(&self, other: &Self) -> bool {
        self.user_id == other.user_id
        && self.created_at == other.created_at
        && self.expires_at == other.expires_at
        && self.token == other.token
    }
}
