use rocket::request::{FromForm, FormItems};
use anyhow::{Context, Result};

pub type UpdateIssue = NewIssue;
pub struct NewIssue {
    pub user_id: i32,
    pub title: String,
    pub content: String,
    pub label_ids: Vec<i32>,
    pub assignee_ids: Vec<i32>,
}

impl NewIssue {
    
}

impl<'f> FromForm<'f> for NewIssue {
    type Error = anyhow::Error;

    fn from_form(items: &mut FormItems<'f>, _strict: bool) -> Result<NewIssue, Self::Error> {
        let mut user_id = -1;
        let (mut title, mut content) = ("".into(), "".into());
        let (mut label_ids, mut assignee_ids) = (Vec::new(), Vec::new());

        for form_item in items {
            let (key, value) = form_item.key_value_decoded();
            match key.as_str() {
                "user_id"   => user_id = value.parse::<i32>()?,
                "title"     => title = value,
                "content"   => content = value,
                "label_ids" => label_ids = value.split(',')
                                    .filter(|l| !l.is_empty())
                                    .map(|l| l.trim().parse::<i32>())
                                    .collect::<Result<Vec<_>, _>>()
                                    .context("Could not parse one or more Label IDs in FromForm for NewIssue.")?,
                "assignee_ids" => assignee_ids = value.split(',')
                                        .filter(|a| !a.is_empty())
                                        .map(|a| a.trim().parse::<i32>())
                                        .collect::<Result<Vec<_>, _>>()
                                        .context("Could not parse one or more Assignee IDs in FromForm for NewIssue.")?,
                _ => (),
            }
        }

        Ok(NewIssue{
            user_id,
            title,
            content,
            label_ids,
            assignee_ids,
        })
    }
}