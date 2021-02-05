use crate::DbConn;
use crate::db;
use anyhow::{Context, Result};

pub type UpdateLabel = Label;
pub type DeleteLabel = Label;
#[derive(FromForm)]
pub struct Label {
    pub id: i32,
    pub project_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub color: String
}

impl Label {
    pub fn update(&self, conn: &DbConn)  -> Result<()> {
        db::Label::update(conn, self)
            .context("Failed to add new Comment")
    }
    
    pub fn delete(&self, conn: &DbConn)  -> Result<()> {
        db::Label::delete(conn, self)
            .context(format!("Failed to delete Label with ID: {}", self.id))
    }
}

impl From<db::Label> for Label {
    fn from(origin: db::Label) -> Label {
        Label {
            id: origin.id,
            project_id: origin.project_id,
            name: origin.name,
            description: origin.description,
            color: origin.color,
        }
    }
}
