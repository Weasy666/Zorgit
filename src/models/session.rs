use rocket::http::{Cookie,Cookies};
use crate::DbConn;
use crate::db;
use crate::models::{Config, User, NewSession};
use chrono::{NaiveDateTime, Utc};
use anyhow::Result;

pub struct Session {
    pub id: i32,
    pub user: User,
    pub token: String,
    pub created_at: NaiveDateTime,
    pub expires: NaiveDateTime,
}

impl Session {
    pub fn all_for_user_id(conn: &DbConn, user_id: i32) -> Result<Vec<Session>> {
        let session = db::Session::all_for_user_id(conn, user_id)?;
        Ok(session)
    }

    pub fn by_token(conn: &DbConn, sid: &str) -> Result<Session> {
        let session = db::Session::by_token(conn, sid)?;
        Ok(session)
    }

    pub fn create(conn: &DbConn, cookies: &mut Cookies<'_>, remember: bool, user_id: i32) -> Result<Session> {
        let (new_session, cookie) = Session::create_session_and_cookie(remember, user_id);

        cookies.add_private(cookie);
        let session = db::Session::insert(conn, &new_session)?;
        Ok(session)
    }

    pub fn update(&self, conn: &DbConn, cookies: &mut Cookies<'_>, remember: bool) -> Result<()> {
        let (update_session, cookie) = Session::create_session_and_cookie(remember, self.user.id);

        cookies.add_private(cookie);
        let result = db::Session::update(conn, &update_session)?;
        Ok(result)
    }

    pub fn delete(&self, conn: &DbConn) -> Result<()> {
        let result = db::Session::delete_with_id(conn, &self.token)?;
        Ok(result)
    }

    fn create_session_and_cookie(remember: bool, user_id: i32) -> (NewSession, Cookie<'static>) {
        let mut new_session = NewSession::new(user_id, 1);
        let mut cookie = Cookie::build(
                Config::global().session_key(),
                new_session.token.clone(),
            )
            .path("/");
        if remember {
            new_session.set_duration(Config::global().session_duration());
            //TODO: migrate cookie-rs' usage of time to chrono, if that is wanted <- currently blocked by a bug in chrono
            let conv_time = time::Timespec::new(new_session.expires.timestamp(), 0);
            cookie = cookie.expires(time::at_utc(conv_time));
        }
        (new_session, cookie.finish())
    }

    pub fn validate(conn: &DbConn, token: &str) -> Result<Session> {
        let session = Session::by_token(&conn, token)?;

        if session.expires > Utc::now().naive_utc() { Ok(session) } else { Err(anyhow!("Session expired")) }
    }
}

impl From<(db::Session, User)> for Session {
    fn from(origin: (db::Session, User)) -> Session {
        Session {
            id: origin.0.id,
            user: origin.1,
            token: origin.0.token,
            created_at: origin.0.created_at,
            expires: origin.0.expires,
        }
    }
}
