use crate::DbConn;
use crate::db;
use crate::models::{Comment, NewComment, Label, User};
use chrono::prelude::NaiveDateTime;
use anyhow::{Context, Result};

pub struct Issue {
    pub id: i32,
    pub number: i32,
    pub project_id: i32,
    pub user: User,
    pub title: String,
    pub content: String,
    pub labels: Option<Vec<Label>>,
    pub num_comments: i32,
    pub comments: Option<Vec<Comment>>,
    pub assignees: Option<Vec<User>>,
    pub is_closed: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Issue {
    pub fn new_comment(&self, conn: &DbConn, new_comment: &NewComment) -> Result<Comment> {
        db::Issue::new_comment(conn, self, new_comment)
            .context("Failed to add new Comment")
    }

    pub fn delete(&self, conn: &DbConn) -> Result<()> {
        db::Issue::delete(conn, self)
            .context(format!("Failed to delete Issue with ID: {}", self.id))
    }
    

    pub fn rendered_content(&self, root: &str) -> String {
        crate::utils::render::markdown(root, &self.content)
    }
}

impl From<(&DbConn, db::Issue, User)> for Issue {
    fn from(origin: (&DbConn, db::Issue, User)) -> Issue {
        Issue {
            id: origin.1.id,
            number: origin.1.number,
            project_id: origin.1.project_id,
            user: origin.2,
            title: origin.1.title,
            content: origin.1.content,
            labels: db::Label::all_for_issue_id(origin.0, origin.1.id).ok(),
            num_comments: origin.1.num_comments,
            comments: db::Comment::all_for_issue_id(origin.0, origin.1.id).ok(),
            assignees: db::User::all_for_issue_id(origin.0, origin.1.id).ok(),
            is_closed: origin.1.is_closed,
            created_at: origin.1.created_at,
            updated_at: origin.1.updated_at,
        }
    }
}