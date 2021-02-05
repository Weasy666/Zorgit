use crate::utils::crypto;
use chrono::{Duration, NaiveDateTime, Utc};

pub type UpdateSession = NewSession;
//? maybe delete NewSession and incorporate the stuff directly in Session and then allow Session creation
//? only with builder pattern or loading from Db or Cookie
pub struct NewSession {
    pub user_id: i32,
    pub token: String,
    pub expires: NaiveDateTime,
}
impl NewSession {
    pub fn new(user_id: i32, duration: i64) -> NewSession {
        NewSession {
            user_id,
            token: crypto::generate_session_token(),
            expires: Utc::now().naive_utc() + Duration::days(duration)
        }
    }

    pub fn set_duration(&mut self, duration: u64) {
        self.expires = Utc::now().naive_utc() + Duration::days(duration as i64);
    }
}