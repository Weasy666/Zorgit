use rocket::request::{FromForm, FormItems};
use uuid::Uuid;
use anyhow::Context;

pub type UpdateComment = NewComment;
pub struct NewComment {
    pub user_id:        i32,
    pub content:        String,
    pub attachment_ids: Option<Vec<Uuid>>,
}

impl NewComment {
    
}

impl<'f> FromForm<'f> for NewComment {
    type Error = anyhow::Error;

    fn from_form(items: &mut FormItems<'f>, _strict: bool) -> Result<NewComment, Self::Error> {
        let mut user_id = -1;
        let mut content = "".into();
        let mut attachment_ids = Vec::new();

        for form_item in items {
            let (key, value) = form_item.key_value_decoded();
            match key.as_str() {
                "user_id"   => user_id = value.parse::<i32>().context("Could not parse User ID in FromForm for NewComment.")?,
                "content"   => content = value,
                "attachment_ids" => attachment_ids = value.split(',')
                                        .map(|uuid| Uuid::parse_str(uuid.trim()))
                                        .collect::<Result<Vec<_>, _>>()
                                        .context("Could not parse one or more Attachment Uuids in FromForm for NewIssue.")?,
                _ => (),
            }
        }

        Ok(NewComment{
            user_id,
            content,
            attachment_ids: if attachment_ids.is_empty() { None } else { Some(attachment_ids) },
        })
    }
}