use std::sync::Arc;
use data_encoding::HEXLOWER;
use rand::{RngCore, SeedableRng};
use rand_chacha::ChaChaRng;
use rocket::{Request, Rocket, try_outcome, State, request::{self, FromRequest}};
use rocket_airlock::Communicator;
use time::{NumericalDuration, OffsetDateTime};

use crate::{Session, credentials::SessionToken};

//#[database("sessions")]
pub struct SessionStore(Arc<sled::Db>);

impl SessionStore {
    pub fn new_session(&self, user_id: &str, expires_in: Option<time::Duration>) -> Result<Session, Box<dyn std::error::Error>> {
        let session = Session {
            user_id: user_id.to_string(),
            created_at: OffsetDateTime::now_utc(),
            expires_at: OffsetDateTime::now_utc() + expires_in.unwrap_or(1.days()),
            token: SessionStore::get_token(),
        };

        let mut key = session.user_id.as_bytes().to_vec();
        key.push(0xff);
        match &session.token {
            SessionToken::Hashed(token) => key.extend_from_slice(token.as_bytes()),
            SessionToken::Plain(_) => Err("")?,
        }

        self.0.insert(key, bincode::serialize(&session)?)?;
        Ok(session)
    }

    pub fn session_by_token(&self, user_id: &str, plain_token: SessionToken) -> Result<Option<Session>, Box<dyn std::error::Error>> {
        let mut prefix = user_id.as_bytes().to_vec();
        prefix.push(0xff);

        let mut sess = None;
        while let Some(Ok(value)) = self.0.scan_prefix(&prefix).values().next() {
            let session: Session = bincode::deserialize(&value)?;
            if session.validate(&plain_token)? {
                sess = Some(session);
                break;
            }
        }

        Ok(sess)
    }

    pub fn is_valid_token(&self, user_id: &str, plain_token: SessionToken) -> Result<bool, Box<dyn std::error::Error>> {
        let mut prefix = user_id.as_bytes().to_vec();
        prefix.push(0xff);

        while let Some(Ok(value)) = self.0.scan_prefix(&prefix).values().next() {
            let session: Session = bincode::deserialize(&value)?;
            if session.validate(&plain_token)? {
                return Ok(true);
            }
        }

        Ok(false)
    }

    pub fn is_valid_session(&self, session: Session) -> Result<bool, Box<dyn std::error::Error>> {
        let mut key = session.user_id.as_bytes().to_vec();
        key.push(0xff);
        match &session.token {
            SessionToken::Hashed(token) => key.extend_from_slice(token.as_bytes()),
            SessionToken::Plain(_) => Err("")?,
        }

        // (userid_sessionid, session)
        if let Some(db_session) = self.0.get(key)? {
            let db_session: Session = bincode::deserialize(&db_session)?;
            Ok(session == db_session && db_session.expires_at >= OffsetDateTime::now_utc())
        } else {
            Ok(false)
        }
    }

    pub fn is_expired_session(&self, session: Session) -> Result<bool, Box<dyn std::error::Error>> {
        let mut key = session.user_id.as_bytes().to_vec();
        key.push(0xff);
        match session.token {
            SessionToken::Hashed(token) => key.extend_from_slice(token.as_bytes()),
            SessionToken::Plain(_) => Err("")?,
        }

        // (userid_sessionid, session)
        if let Some(db_session) = self.0.get(key)? {
            let db_session: Session = bincode::deserialize(&db_session)?;
            Ok(db_session.expires_at >= OffsetDateTime::now_utc())
        } else {
            Ok(true)
        }
    }

    fn get_token() -> SessionToken {
        let mut rnd = ChaChaRng::from_entropy();
        let mut token = [0u8; 32];
        rnd.fill_bytes(&mut token);

        SessionToken::Plain(HEXLOWER.encode(&token))
    }
}

#[rocket::async_trait]
impl Communicator for SessionStore {
    async fn from(rocket: &Rocket) -> Result<Self, Box<dyn std::error::Error>> {
        let db = rocket.state::<SessionStore>().unwrap();
        Ok(SessionStore(db.0.clone()))
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for SessionStore {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let db = try_outcome!(request.guard::<State<crate::SessionStore>>().await);
        request::Outcome::Success(SessionStore(db.0.clone()))
    }
}

// #[database("sqlite")]
// pub struct Database(rusqlite::Connection);

// pub struct SecComm;

// impl SecComm {
//     async fn connect(rocket: &Rocket) -> Option<Database> {
//         Database::get_one(&rocket).await
//     }
// }

// #[rocket::async_trait]
// impl Communicator for SecComm {
//     async fn from(_rocket: &Rocket) -> Result<Self, Box<dyn std::error::Error>> {
//         Ok(SecComm)
//     }
// }

// pub struct Sled(pub sled::Db);

// //TODO: Replace sqlite with sled
// impl Poolable for Sled {
//     type Manager = SledConnectionManager;
//     type Error = std::convert::Infallible;

//     fn pool(db_name: &str, rocket: &rocket::Rocket) -> PoolResult<Self> {
//         let config = databases::Config::from(db_name, rocket)?;
//         let manager = SledConnectionManager::file(&*config.url);
//         Ok(r2d2::Pool::builder().max_size(config.pool_size).build(manager)?)
//     }
// }

// impl std::ops::Deref for Sled {
//     type Target = sled::Db;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// impl std::ops::DerefMut for Sled {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0
//     }
// }

// #[derive(Debug)]
// enum Source {
//     File(PathBuf),
//     //Memory
// }

// /// A [`r2d2::ManageConnection`] implementation for [`sled`].
// /// It is more or less just a thin wrapper around [`sled::Db`].
// ///
// /// [`r2d2::ManageConnection`]: https://docs.rs/r2d2/0.8/r2d2/trait.ManageConnection.html
// /// [`sled`]: https://docs.rs/sled/0.34/sled/
// /// [`sled::Db`]: https://docs.rs/sled/0.34/sled/struct.Db.html
// pub struct SledConnectionManager {
//     source: Source,
// }

// impl SledConnectionManager {
//     /// Creates a new `SledConnectionManager` from file.
//     ///
//     /// For more information see [`sled::open`]
//     ///
//     /// [`sled::open`]: https://docs.rs/sled/0.34/sled/fn.open.html
//     pub fn file<P: AsRef<Path>>(path: P) -> Self {
//         Self {
//             source: Source::File(path.as_ref().to_path_buf()),
//         }
//     }
// }

// impl r2d2::ManageConnection for SledConnectionManager {
//     type Connection = Sled;
//     type Error = sled::Error;

//     fn connect(&self) -> Result<Sled, sled::Error> {
//         match self.source {
//             Source::File(ref path) => sled::open(path).map(Sled)
//         }
//     }

//     fn is_valid(&self, conn: &mut Sled) -> Result<(), sled::Error> {
//         // Should hopefully check that we are still connected to the db.
//         conn.0.first().map(|_| ())
//     }

//     fn has_broken(&self, _: &mut Sled) -> bool {
//         false
//     }
// }
