use rocket::request::{FromForm, FormItems};
use diesel::prelude::*;
use crate::models::{Email, Password};
use crate::db::schema::*;
use std::path::PathBuf;

pub struct NewUser {
    pub types: i16,
    pub email: Email,
    pub username: String,
    pub avatar: PathBuf,
    password: Password,
    pub is_admin: bool,
}

impl NewUser {
    /// Creates a new user with the given values. The given password is encoded with argon2 and a salt.
    pub fn new(types: i16, email: Email, username: &str, avatar: Option<PathBuf>, password: &str, is_admin: bool,
    ) -> Self {
        let avatar = if avatar.is_none() { crate::utils::create_default_avatar(&email.address).ok() } else { avatar };

        NewUser {
            types,
            email: email.as_primary(true),
            username: username.to_string(),
            avatar: avatar.unwrap(),
            password: password.parse::<Password>().unwrap(),
            is_admin,
        }
    }
}

impl<'a> Insertable<users::table> for &'a NewUser {
    #![allow(clippy::type_complexity)]
    type Values = <(
        diesel::dsl::Eq<users::types, &'a i16>,
        diesel::dsl::Eq<users::username, &'a String>,
        Option<diesel::dsl::Eq<users::avatar, String>>,
        diesel::dsl::Eq<users::password, String>,
        diesel::dsl::Eq<users::salt, String>,
        diesel::dsl::Eq<users::is_admin, bool>,
    ) as Insertable<users::table>>::Values;

    fn values(self) -> Self::Values {
        let password = self.password.clone().hash();
        (
            users::types.eq(&self.types),
            users::username.eq(&self.username),
            self.avatar.to_str().map(ToString::to_string).map(|a| users::avatar.eq(a)),
            users::password.eq(password.password().unwrap()),
            users::salt.eq(password.salt().unwrap()),
            users::is_admin.eq(self.is_admin),
        ).values()
    }
}

impl<'f> FromForm<'f> for NewUser {
    type Error = anyhow::Error;

    fn from_form(items: &mut FormItems<'f>, _strict: bool) -> Result<NewUser, Self::Error> {
        let mut email = None;
        let mut username = String::default();
        let mut avatar = None;
        let mut password = String::default();

        for form_item in items {
            let (key, value) = form_item.key_value_decoded();
            match key.as_str() {
                "email"    => email = value.parse::<Email>()?.into(),
                "username" => username = value,
                "avatar"   => avatar = PathBuf::from(value).into(),
                "password" => password = value,
                _ => (),
            }
        }

        Ok(NewUser::new(
            0,
            email.expect("No Email provided in from_form of NewUser."),
            &username,
            avatar,
            &password,
            false,
        ))
    }
}