use rocket::request::{FromForm, FormItems};
use crate::models::{Email, Language, DotFile, Url};
use anyhow::Context;


pub struct UpdateProfile {
    pub id: i32,
    pub avatar: DotFile,
    pub avatar_email: Option<Email>,
    pub email: Email,
    pub location: Option<String>,
    pub website: Option<Url>,
    pub description: Option<String>,
    pub language: Language,
    pub is_email_hidden: bool,
}

impl<'f> FromForm<'f> for UpdateProfile {
    type Error = anyhow::Error;

    fn from_form(items: &mut FormItems<'f>, _strict: bool) -> Result<UpdateProfile, Self::Error> {
        let mut id = -1;
        let mut avatar = DotFile::from("/");
        let mut avatar_email = None;
        let mut email = None;
        let mut location = None;
        let mut website = None;
        let mut description = None;
        let mut language = "en-EN".parse::<Language>()?;
        let mut is_email_hidden = true;

        for form_item in items {
            let (key, value) = form_item.key_value_decoded();
            match key.as_str() {
                "id" => id = value.parse::<i32>().context("Could not parse User ID in FromForm for UpdateProfile.")?,
                "avatar" if !value.is_empty() => avatar = DotFile::from(value).into(),
                "avatar_email" if !value.is_empty() => avatar_email = value.parse::<Email>()?.into(),
                "email"    => email = value.parse::<Email>()?.as_primary(true).into(),
                "location" if !value.is_empty() => location = value.into(),
                "website" if !value.is_empty() => website = value.parse::<Url>().context("Could not parse Url in FromForm for UpdateProfile.")?.into(),
                "description" if !value.is_empty() => description = value.into(),
                "language" => language = value.parse::<Language>()?,
                "show_email" => is_email_hidden = value == "off",
                _ => (),
            }
        }

        Ok(UpdateProfile {
            id,
            avatar,
            avatar_email,
            email: email.expect("No viable email found"),
            location,
            website,
            description,
            language,
            is_email_hidden,
        })
    }
}