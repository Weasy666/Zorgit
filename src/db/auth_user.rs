use crate::DbConn;
use crate::db;
use crate::models::{self, Email, Language, Password, Url};
use diesel::prelude::*;
use chrono::NaiveDateTime;
use crate::db::schema::*;
use std::path::PathBuf;

pub struct AuthUser {
    pub id: i32,
    pub types: i16,
    pub username: String,
    pub full_name: Option<String>,
    pub avatar: PathBuf,
    pub avatar_email: Option<Email>,
    pub password: Password,
    pub location: Option<String>,
    pub website: Option<Url>,
    pub description: Option<String>,
    pub language: Language,
    pub must_change_password: bool,
    pub is_email_hidden: bool,
    pub is_admin: bool,
    pub is_organisation: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub last_seen_at: NaiveDateTime,
}

impl Queryable<users::SqlType, diesel::sqlite::Sqlite> for AuthUser {
    type Row = (i32,i16,String,Option<String>,String,Option<String>,String,String,Option<String>,Option<String>,Option<String>,
                String,bool,bool,bool,bool,NaiveDateTime,NaiveDateTime,NaiveDateTime);

    fn build((id, types, username, full_name, avatar, avatar_email, password, salt, location, website, description, language,
            must_change_password, is_email_hidden, is_admin, is_organisation, created_at, updated_at, last_seen_at): Self::Row
    ) -> Self {
        Self {
            id,
            types,
            username,
            full_name,
            avatar: PathBuf::from(avatar),
            avatar_email: avatar_email.map(|a| a.parse::<Email>().expect(&format!("Not a valid email address: {}", a))),
            password: Password::Hashed(format!("{}${}", salt, password)),
            location,
            website: website.map(|w| w.parse::<Url>().expect("Invalid url")),
            description,
            language: language.parse::<Language>().expect(&format!("Not a valid Language Identifier: {}", language)),
            must_change_password,
            is_email_hidden,
            is_admin,
            is_organisation,
            created_at,
            updated_at,
            last_seen_at,
        }
    }
}

impl AuthUser {
    pub fn by_name_or_email(conn: &DbConn, keyword: &str) -> QueryResult<models::AuthUser> {
        users::table.inner_join(emails::table)
            .filter(users::username.eq(keyword).or(emails::address.eq(keyword)))
            .select((users::all_columns, emails::all_columns))
            .first::<(db::AuthUser, db::Email)>(&conn.0)
            .map(|(a, e)| (a, e).into())
    }
}