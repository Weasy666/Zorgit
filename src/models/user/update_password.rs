use rocket::request::{FromForm, FormItems};
use anyhow::Context;
use crate::models::Password;


pub struct UpdatePassword {
    pub id: i32,
    pub old: Password,
    pub new: Password,
}

impl<'f> FromForm<'f> for UpdatePassword {
    type Error = anyhow::Error;

    fn from_form(items: &mut FormItems<'f>, _strict: bool) -> Result<UpdatePassword, Self::Error> {
        let mut id = -1;
        let mut password_old = Err(anyhow!(""));
        let mut password_new = Err(anyhow!(""));
        let mut password_repeat = Err(anyhow!(""));

        for form_item in items {
            let (key, value) = form_item.key_value_decoded();
            match key.as_str() {
                "id" => id = value.parse::<i32>().context("Could not parse User ID in FromForm for UpdatePassword.")?,
                "password_old" if !value.is_empty() => password_old = value.parse::<Password>(),
                "password_new" if !value.is_empty() => password_new = value.parse::<Password>(),
                "password_repeat" if !value.is_empty() => password_repeat = value.parse::<Password>(),
                _ => (),
            }
        }

        if password_new.is_err() || password_new.is_err() || password_repeat.is_err()
            || password_new.as_ref().unwrap() != password_repeat.as_ref().unwrap() {
            return Err(anyhow!("Empty new password or not equal to repeat password in FromForm for UpdatePassword."));
        }

        Ok(UpdatePassword {
            id,
            old: password_old?,
            new: password_new?,
        })
    }
}