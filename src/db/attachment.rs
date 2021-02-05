use crate::DbConn;
use diesel::prelude::*;
use crate::db;
use crate::db::schema::*;
use chrono::prelude::*;
use uuid::Uuid;
use crate::models;

#[derive(Identifiable, Associations, Clone)]
#[belongs_to(db::Comment)]
#[belongs_to(db::Issue)]
#[belongs_to(db::User)]
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
    pub fn all_for_comment(conn: &DbConn, comment: &models::Comment) -> QueryResult<Vec<models::Attachment>> {
        Attachment::all_for_comment_id(conn, comment.id)
    }

    pub fn all_for_comment_id(conn: &DbConn, comment_id: i32) -> QueryResult<Vec<models::Attachment>> {
        attachments::table.filter(attachments::comment_id.eq(comment_id))
            .load::<Attachment>(&conn.0)
            .map(|v| v.into_iter().map(|a| a.into()).collect::<Vec<_>>())
    }

    pub fn by_uuid(conn: &DbConn, uuid: &Uuid) -> QueryResult<models::Attachment> {
        attachments::table.filter(attachments::uuid.eq(uuid.to_simple().to_string()))
            .first::<Attachment>(&conn.0)
            .map(|a| a.into())
    }

    pub fn insert_all_for_comment_id(conn: &DbConn, comment_id: i32, attachments: &[models::Attachment]) -> QueryResult<Vec<models::Attachment>> {
        let attachments = attachments.iter()
            .flat_map(|a| Attachment::insert_for_comment_id(conn, comment_id, a))
            .collect::<Vec<_>>();

        Ok(attachments)
    }

    pub fn insert_for_comment_id(conn: &DbConn, comment_id: i32, attachment: &models::Attachment) -> QueryResult<models::Attachment> {
        conn.0.transaction::<_, diesel::result::Error, _>(|| {
            diesel::update(attachments::table.filter(attachments::id.eq(attachment.id)))
                .set(attachments::comment_id.eq(comment_id))
                .execute(&conn.0)?;

            let mut attachment = attachment.clone();
            attachment.comment_id = Some(comment_id);
            Ok(attachment)
        })
    }

    pub fn delete(conn: &DbConn, attachment: &models::Attachment) -> QueryResult<()> {
        diesel::delete(attachments::table.find(attachment.id))
            .execute(&conn.0)
            .map(|_| ())
    }
}

impl Queryable<attachments::SqlType, diesel::sqlite::Sqlite> for Attachment {
    #![allow(clippy::type_complexity)]
    type Row = (i32,String,i32,Option<i32>,Option<i32>,Option<i32>,String,i32,NaiveDateTime,NaiveDateTime);

    fn build((id, uuid, user_id, project_id, issue_id, comment_id, name, download_count, created_at, updated_at): Self::Row
    ) -> Self {
        Self {
            id,
            uuid: Uuid::parse_str(&uuid).expect(&format!("Unable to parse Uuid: {}", &uuid)),
            user_id,
            project_id,
            issue_id,
            comment_id,
            name,
            download_count,

            created_at,
            updated_at,
        }
    }
}