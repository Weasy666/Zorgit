use crate::DbConn;
use crate::db;
use crate::models;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use crate::db::schema::*;

#[derive(Queryable, Associations, Insertable)]
#[belongs_to(db::User)]
pub struct Session {
    pub id: i32,
    pub user_id: i32,
    pub token: String,
    pub created_at: NaiveDateTime,
    pub expires: NaiveDateTime,
}

impl Session {
    pub fn all_for_user_id(conn: &DbConn, user_id: i32) -> QueryResult<Vec<models::Session>> {
        sessions::table.filter(sessions::user_id.eq(user_id))
            .inner_join(users::table.inner_join(emails::table))
            .select((sessions::all_columns, users::all_columns, emails::all_columns))
            .load::<(db::Session, db::User, db::Email)>(&conn.0)
            .map(|v| v.into_iter().map(|(s, u, e)| (s, (u, e).into()).into()).collect::<Vec<_>>())
    }

    pub fn by_token(conn: &DbConn, token: &str) -> QueryResult<models::Session> {
        sessions::table.filter(sessions::token.eq(token))
            .inner_join(users::table.inner_join(emails::table))
            .select((sessions::all_columns, users::all_columns, emails::all_columns))
            .first::<(db::Session, db::User, db::Email)>(&conn.0)
            .map(|(s, u, e)| (s, (u, e).into()).into())
    }

    pub fn insert(conn: &DbConn, new_session: &models::NewSession) -> QueryResult<models::Session> {
        conn.0.transaction::<_, diesel::result::Error, _>(|| {
            diesel::insert_into(sessions::table)
                .values((
                    sessions::user_id.eq(new_session.user_id),
                    sessions::token.eq(&new_session.token),
                    sessions::expires.eq(new_session.expires),
                ))
                .execute(&conn.0)?;
            Session::by_token(conn, &new_session.token)
        })
    }

    pub fn update(conn: &DbConn, update_session: &models::UpdateSession) -> QueryResult<()> {
        diesel::update(sessions::table.filter(sessions::user_id.eq(update_session.user_id)))
            .set((
                sessions::token.eq(&update_session.token),
                sessions::expires.eq(update_session.expires)
            ))
            .execute(&conn.0)
            .map(|_| ())
    }

    pub fn delete_with_id(conn: &DbConn, sid: &str) -> QueryResult<()> {
        diesel::delete(sessions::table.filter(sessions::token.eq(sid)))
            .execute(&conn.0)
            .map(|_| ())
    }
}