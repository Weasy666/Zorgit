use crate::models::Password;
use rocket::request::{FromForm, FormItems};
use anyhow::Context;


pub struct UpdateAccount {
    pub id: i32,
    pub username: String,
    pub password: Password,
}

impl<'f> FromForm<'f> for UpdateAccount {
    type Error = anyhow::Error;

    fn from_form(items: &mut FormItems<'f>, _strict: bool) -> Result<UpdateAccount, Self::Error> {
        let mut id = -1;
        let mut username = String::default();
        let mut password = Err(anyhow!(""));

        for form_item in items {
            let (key, value) = form_item.key_value_decoded();
            match key.as_str() {
                "id" => id = value.parse::<i32>().context("Could not parse User ID in FromForm for UpdateAccount.")?,
                "username" => username = value.into(),
                "password" if !value.is_empty() => password = value.parse::<Password>(),
                _ => (),
            }
        }

        if password.is_err() || username.is_empty() {
            return Err(anyhow!("Empty password or username in FromForm for UpdateAccount."));
        }

        Ok(UpdateAccount {
            id,
            username,
            password: password?,
        })
    }
}