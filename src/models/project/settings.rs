use crate::models::Url;
use rocket::request::{FromForm, FormItems};
use anyhow::{Context, Result};
use crate::utils::IntoOption;

pub struct BasicSettings {
    pub project_id: i32,
    pub project_name: String,
    pub is_private: bool,
    pub description: Option<String>,
    pub website: Option<Url>,
    pub topics: Option<Vec<i32>>,
}

impl<'f> FromForm<'f> for BasicSettings {
    type Error = anyhow::Error;

    fn from_form(items: &mut FormItems<'f>, _strict: bool) -> Result<BasicSettings, Self::Error> {
        let mut project_id = -1;
        let mut project_name = String::default();
        let mut is_private = true;
        let mut description = None;
        let mut website = None;
        let mut topics = None;

        for form_item in items {
            let (key, value) = form_item.key_value_decoded();
            match key.as_str() {
                "id" => project_id = value.parse::<i32>().context("Could not parse Project ID in FromForm for BasicSettings.")?,
                "project_name" => project_name = value.into(),
                "is_private" => is_private = value.parse::<bool>().context("Could not parse is_private in FromForm for BasicSettings.")?,
                "description" if !value.is_empty() => description = value.into(),
                "website" if !value.is_empty() => website = value.parse::<Url>().context("Could not parse Website Url in FromForm for BasicSettings.")?.into(),
                "topic_ids" if !value.is_empty() => topics = value.split(',')
                                                        .map(|id| id.trim().parse::<i32>())
                                                        .collect::<Result<Vec<i32>, _>>()
                                                        .context("Could not parse one or more Topic IDs in FromForm for BasicSettings.")?
                                                        .into_option(),
                _ => (),
            }
        }

        if project_id <= 0 || project_name.is_empty() {
            return Err(anyhow!("Wrong Project ID or empty Projectname in FromForm for BasicSettings."));
        }

        Ok(BasicSettings {
            project_id,
            project_name,
            is_private,
            description,
            website,
            topics,
        })
    }
}

pub struct ProjectTransfer {
    pub new_owner: String,
    pub confirm_name: String,
}

impl<'f> FromForm<'f> for ProjectTransfer {
    type Error = anyhow::Error;

    fn from_form(items: &mut FormItems<'f>, _strict: bool) -> Result<ProjectTransfer, Self::Error> {
        let mut new_owner = String::default();
        let mut confirm_name = String::default();

        for form_item in items {
            let (key, value) = form_item.key_value_decoded();
            match key.as_str() {
                "new_owner" => new_owner = value.into(),
                "confirm_name" => confirm_name = value.into(),
                _ => (),
            }
        }

        if new_owner.is_empty() || confirm_name.is_empty() {
            return Err(anyhow!("Empty new_owner or empty confirm_name in FromForm for ProjectTransfer."));
        }

        Ok(ProjectTransfer {
            new_owner,
            confirm_name,
        })
    }
}

pub struct ProjectDelete {
    pub id: i32,
    pub confirm_name: String,
}

impl<'f> FromForm<'f> for ProjectDelete {
    type Error = anyhow::Error;

    fn from_form(items: &mut FormItems<'f>, _strict: bool) -> Result<ProjectDelete, Self::Error> {
        let mut id = -1;
        let mut confirm_name = String::default();

        for form_item in items {
            let (key, value) = form_item.key_value_decoded();
            match key.as_str() {
                "id" => id = value.parse::<i32>().context("Could not parse Project ID in FromForm for ProjectDelete.")?,
                "confirm_name" => confirm_name = value.into(),
                _ => (),
            }
        }

        if id <= 0 || confirm_name.is_empty() {
            return Err(anyhow!("Wrong Project ID or empty Projectname in FromForm for ProjectDelete."));
        }

        Ok(ProjectDelete {
            id,
            confirm_name,
        })
    }
}