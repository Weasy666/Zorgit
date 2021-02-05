use crate::db;
use crate::DbConn;
use crate::models::{Attachment, User};
use chrono::prelude::*;
use std::ops::{Deref, DerefMut};
use anyhow::{Context, Result};

#[derive(Debug, Clone)]
pub enum Comment {
    Plain(CommentInner),
    CommentRef(CommentInner),
    CommitRef(CommentInner),
    IssueRef(CommentInner),
    Err(CommentInner),
}

impl Comment {
    pub fn delete(&self, conn: &DbConn) -> Result<()> {
        db::Comment::delete(conn, self)
            .context(format!("Failed to delete Comment with ID: {}", self.id))
    }

    pub fn rendered_content(&self, root: &str) -> String {
        crate::utils::render::markdown(root, &self.content)
    }

    pub fn created_pretty_time(&self) -> String {
        self.created_at.format("%a, %e %b %Y, %T").to_string()
    }

    pub fn updated_pretty_time(&self) -> String {
        self.updated_at.format("%a, %e %b %Y, %T").to_string()
    }
}

impl From<(db::Comment, User, Option<Vec<Attachment>>)> for Comment {
    fn from(origin: (db::Comment, User, Option<Vec<Attachment>>)) -> Self {
        let inner = CommentInner {
            id: origin.0.id,
            issue_id: origin.0.issue_id,
            user: origin.1,
            content: origin.0.content,
            attachments: origin.2,
            created_at: origin.0.created_at,
            updated_at: origin.0.updated_at,
        };
        match origin.0.enum_type {
            0 => Comment::Plain(inner),
            1 => Comment::CommentRef(inner),
            2 => Comment::CommitRef(inner),
            3 => Comment::IssueRef(inner),
            _ => Comment::Err(inner),
        }
    }
}

impl Deref for Comment {
    type Target = CommentInner;

    fn deref(&self) -> &Self::Target {
        match self {
            Comment::Plain(c)        => c,
            Comment::CommentRef(c)   => c,
            Comment::CommitRef(c)    => c,
            Comment::IssueRef(c)     => c,
            Comment::Err(c)     => c,
        }
    }
}

impl DerefMut for Comment {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Comment::Plain(c)        => c,
            Comment::CommentRef(c)   => c,
            Comment::CommitRef(c)    => c,
            Comment::IssueRef(c)     => c,
            Comment::Err(c)     => c,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CommentInner {
    pub id:         i32,
    pub issue_id:   i32,
    pub user:       User,
    pub content:    String,
    pub attachments: Option<Vec<Attachment>>,
    
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
