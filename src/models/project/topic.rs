use rocket::request::{FromForm, FormItems};
use anyhow::{Context, Result};
use std::vec::Vec;
use serde::Serialize;
use crate::db;


#[derive(Clone, Serialize)]
pub struct Topic {
    pub id: i32,
    pub name: String,
}

pub struct Topics<T> {
    pub values: Vec<T>,
}

impl From<db::Topic> for Topic {
    fn from(origin: db::Topic) -> Topic {
        Topic {
            id: origin.id,
            name: origin.name,
        }
    }
}

impl<'f> FromForm<'f> for Topics<String> {
    type Error = anyhow::Error;

    fn from_form(items: &mut FormItems<'f>, _strict: bool) -> Result<Topics<String>, Self::Error> {
        let mut topics = Vec::new();

        for form_item in items {
            let (key, value) = form_item.key_value_decoded();
            match key.as_str() {
                "topic_names" => topics = value.split(',')
                                    .map(|t| t.trim())
                                    .map(ToString::to_string)
                                    .collect::<Vec<_>>(),
                _ => (),
            }
        }

        Ok(Topics { values: topics })
    }
}

impl<'f> FromForm<'f> for Topics<i32> {
    type Error = anyhow::Error;

    fn from_form(items: &mut FormItems<'f>, _strict: bool) -> Result<Topics<i32>, Self::Error> {
        let mut topics = Vec::new();

        for form_item in items {
            let (key, value) = form_item.key_value_decoded();
            match key.as_str() {
                "topic_ids" => topics = value.split(',')
                                    .map(|t| t.trim().parse::<i32>())
                                    .collect::<Result<Vec<_>, _>>()
                                    .context("Could not parse one or more Topic IDs in FromForm for Topics<i32>.")?,
                _ => (),
            }
        }

        Ok(Topics { values: topics })
    }
}