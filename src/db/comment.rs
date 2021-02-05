use crate::DbConn;
use diesel::prelude::*;
use crate::db;
use crate::models;
use crate::db::schema::*;
use chrono::prelude::*;

#[derive(Identifiable, Associations, Queryable)]
#[belongs_to(db::Issue)]
#[belongs_to(db::User)]
pub struct Comment {
    pub id:         i32,
    pub enum_type:  i16,
    pub issue_id:   i32,
    pub user_id:    i32,
    pub content:    String,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Comment {
    pub fn by_id(conn: &DbConn, comment_id: i32) -> QueryResult<models::Comment> {
        comments::table.find(comment_id)
            .inner_join(users::table.inner_join(emails::table))
            .filter(emails::is_primary.eq(true))
            .select((comments::all_columns, users::all_columns, emails::all_columns))
            .first::<(db::Comment, db::User, db::Email)>(&conn.0)
            .map(|(c, u, e)| (c, (u, e).into(), None).into())
    }

    pub fn all_for_issue(conn: &DbConn, issue: &models::Issue) -> QueryResult<Vec<models::Comment>> {
        Comment::all_for_issue_id(conn, issue.id)
    }

    pub fn all_for_issue_id(conn: &DbConn, issue_id: i32) -> QueryResult<Vec<models::Comment>> {
        comments::table.filter(comments::issue_id.eq(issue_id))
            .inner_join(users::table.inner_join(emails::table))
            .filter(emails::is_primary.eq(true))
            .select((comments::all_columns, users::all_columns, emails::all_columns))
            .load::<(db::Comment, db::User, db::Email)>(&conn.0)
            .map(|v| v.into_iter().map(|(c, u, e)| (c, (u, e).into(), None).into()).collect::<Vec<_>>())
    }

    pub fn delete(conn: &DbConn, comment: &models::Comment) -> QueryResult<()> {
        conn.0.transaction::<_, diesel::result::Error, _>(|| {
            if comment.attachments.is_some() {
                for attachment in comment.attachments.as_ref().unwrap() {
                    attachment.delete(conn).map_err(|e| panic!(e)).unwrap();
                }
            }

            diesel::delete(comments::table.find(comment.id))
                .execute(&conn.0)
                .map(|_| ())
        })
    }
}