use crate::models;
use crate::DbConn;
use diesel::prelude::*;
use crate::db::schema::*;
use chrono::{NaiveDateTime, Utc};

pub type Email = crate::models::Email;

impl Email {
    pub fn new_for_user(conn: &DbConn, user: &models::User, new_email: &models::NewEmail) -> QueryResult<models::Email> {
        conn.0.transaction::<_, diesel::result::Error, _>(|| {
            let notification: i16 = new_email.notification.clone().into();
            let token = format!("{}{}{}", user.username, user.id, new_email.address);
            diesel::insert_into(emails::table)
                .values((
                    emails::user_id.eq(&user.id),
                    emails::address.eq(&new_email.address),
                    emails::is_primary.eq(&new_email.is_primary),
                    emails::token.eq(models::Sha1::new(token).to_string()),
                    emails::token_created_at.eq(Utc::now().naive_utc()),
                    emails::notification.eq(&notification),
                ))
                .execute(&conn.0)?;
            
            emails::table.filter(emails::address.eq(&new_email.address))
                .first::<Email>(&conn.0)
        })
    }

    pub fn activate_for_user_with_token(conn: &DbConn, user: &models::User, token: &models::Sha1) -> QueryResult<()> {
        let none_string: Option<String> = None;
        let none_time: Option<NaiveDateTime> = None;
        diesel::update(emails::table.filter(emails::user_id.eq(user.id).and(emails::token.eq(&token.to_string()))))
            .set((
                emails::token.eq(none_string),
                emails::token_created_at.eq(none_time),
                emails::activated_at.eq(Utc::now().naive_utc()),
            ))
            .execute(&conn.0)
            .map(|_| ())
    }

    pub fn activate_for_user(conn: &DbConn, user: &models::User, email: &models::Email) -> QueryResult<()> {
        let none_string: Option<String> = None;
        let none_time: Option<NaiveDateTime> = None;
        diesel::update(emails::table.filter(emails::user_id.eq(user.id).and(emails::id.eq(email.id))))
            .set((
                emails::token.eq(none_string),
                emails::token_created_at.eq(none_time),
                emails::activated_at.eq(Utc::now().naive_utc()),
            ))
            .execute(&conn.0)
            .map(|_| ())
    }
}

impl Queryable<emails::SqlType, diesel::sqlite::Sqlite> for Email {
    type Row = (i32,i32,String,bool,Option<NaiveDateTime>,Option<String>,Option<NaiveDateTime>,i16);

    fn build((id, _user_id, address, is_primary, activated_at, token, token_created_at, notification): Self::Row
    ) -> Self {
        Self {
            id,
            address,
            is_primary,
            activated_at,
            token: token.map(|t| t.parse::<models::Sha1>().expect("Not a valid Sha1 value.")),
            token_created_at,
            notification: models::Notification::from(notification),
        }
    }
}
