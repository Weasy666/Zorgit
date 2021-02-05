use rocket::request::FromForm;
use crate::db::schema::*;

#[derive(Insertable, Clone, FromForm)]
#[table_name="labels"]
pub struct NewLabel {
    pub name: String,
    pub description: Option<String>,
    pub color: String
}

impl NewLabel {
    
}