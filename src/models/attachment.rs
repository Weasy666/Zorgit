use chrono::prelude::*;
use uuid::Uuid;
use diesel::Connection;
use crate::db;
use crate::DbConn;
use crate::models::Project;
use anyhow::{Context, Result};

#[derive(Debug, Clone)]
pub struct Attachment {
    pub id:             i32,
    pub uuid:           Uuid,
    pub user_id:        i32,
    pub project_id:     Option<i32>,
    pub issue_id:       Option<i32>,
    pub comment_id:     Option<i32>,
    pub name:           String,
    pub download_count: i32,
    
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Attachment {
    pub fn delete(&self, conn: &DbConn) -> Result<()> {
        conn.0.transaction::<_, anyhow::Error, _>(|| {
            db::Attachment::delete(conn, self)?;

            let mut project_id = -1;
            if self.project_id.is_some() {
                project_id = self.project_id.expect("Checked for Some, so should be safe.");
            }
            else if self.issue_id.is_some() {
                let issue = db::Issue::by_id(conn, self.issue_id.expect("Checked for Some, so should be safe."))?;
                project_id = issue.project_id;
            }
            else if self.comment_id.is_some() {
                let comment = db::Comment::by_id(conn, self.comment_id.expect("Checked for Some, so should be safe."))?;
                let issue = db::Issue::by_id(conn, comment.issue_id)?;
                project_id = issue.project_id;
            }

            if project_id == -1 {
                return Err(anyhow!("Orphaned attachment. This should not be possible."));
            }

            let (projectname, ownername) = db::Project::name_and_ownername_by_id(conn, project_id)?;
            let attachment_path = Project::compose_attachments_dir(&ownername, &projectname).join(&self.uuid.to_string());
            std::fs::remove_file(&attachment_path).context(format!("Could not remove attachment: {:#?}", attachment_path))?;
            Ok(())
        })
    }
}

impl From<db::Attachment> for Attachment {
    fn from(origin: db::Attachment) -> Self {
        Attachment {
            id: origin.id,
            uuid: origin.uuid,
            user_id: origin.user_id,
            project_id: origin.project_id,
            issue_id: origin.issue_id,
            comment_id: origin.comment_id,
            name: origin.name,
            download_count: origin.download_count,
            
            created_at: origin.created_at,
            updated_at: origin.updated_at,
        }
    }
}