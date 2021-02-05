use crate::DbConn;
use crate::db;
use diesel::prelude::*;
use crate::models;
use crate::db::schema::*;
use chrono::prelude::NaiveDateTime;
use std::collections::HashMap;

#[derive(Identifiable, Associations, Queryable, Clone)]
#[belongs_to(db::Project)]
#[belongs_to(db::User)]
pub struct Issue {
    pub id: i32,
    pub number: i32,
    pub project_id: i32,
    pub user_id: i32,
    pub title: String,
    pub content: String,
    pub num_comments: i32,
    pub is_closed: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Issue {
    pub fn by_id(conn: &DbConn, issue_id: i32) -> QueryResult<models::Issue> {
        issues::table.find(issue_id)
            .inner_join(users::table.inner_join(emails::table))
            .filter(emails::is_primary.eq(true))
            .select((issues::all_columns, users::all_columns, emails::all_columns))
            .first::<(db::Issue, db::User, db::Email)>(&conn.0)
            .map(|(i, u, e)| (conn, i, (u, e).into()).into())
    }

    pub fn all_for_project(conn: &DbConn, project: &models::Project) -> QueryResult<Vec<models::Issue>> {
        issues::table.filter(issues::project_id.eq(project.id))
            .inner_join(users::table.inner_join(emails::table))
            .filter(emails::is_primary.eq(true))
            .select((issues::all_columns, users::all_columns, emails::all_columns))
            .load::<(db::Issue, db::User, db::Email)>(&conn.0)
            .map(|v| v.into_iter().map(|(i, u, e)| (conn, i, (u, e).into()).into()).collect::<Vec<_>>())
    }

    pub fn all_for_user(conn: &DbConn, user: &models::User) -> QueryResult<Vec<models::Issue>> {
        issues::table.filter(issues::user_id.eq(user.id))
            .order(issues::updated_at.desc())
            .select(issues::all_columns)
            .load::<db::Issue>(&conn.0)
            .map(|v| v.into_iter().map(|i| (conn, i, user.clone()).into()).collect::<Vec<_>>())
    }

    pub fn all_for_user_with_projects(conn: &DbConn, user: &models::User) -> QueryResult<(HashMap<i32,models::Project>, Vec<models::Issue>)> {
        conn.0.transaction::<_, diesel::result::Error, _>(|| {
            let issues = issues::table.filter(issues::user_id.eq(user.id))
                            .order(issues::updated_at.desc())
                            .inner_join(projects::table)
                            .select((issues::all_columns, projects::all_columns))
                            .load::<(db::Issue, db::Project)>(&conn.0)?;
            let mut projects = HashMap::new();
            let issues = issues.into_iter().map(|(i, p)| {
                projects.insert(p.id, (p, user.clone(), None).into());
                (conn, i, user.clone()).into()
            })
            .collect::<Vec<_>>();

            Ok((projects , issues))
        })
    }

    pub fn for_project_by_number(conn: &DbConn, project: &models::Project, issue_num: i32) -> QueryResult<models::Issue> {
        issues::table.filter(issues::project_id.eq(project.id).and(issues::number.eq(issue_num)))
            .inner_join(users::table.inner_join(emails::table))
            .filter(emails::is_primary.eq(true))
            .select((issues::all_columns, users::all_columns, emails::all_columns))
            .first::<(db::Issue, db::User, db::Email)>(&conn.0)
            .map(|(i, u, e)| (conn, i, (u, e).into()).into())
    }

    pub fn new_comment(conn: &DbConn, issue: &models::Issue, new_comment: &models::NewComment) -> QueryResult<models::Comment> {
        conn.0.transaction::<_, diesel::result::Error, _>(|| {
            diesel::update(issues::table.filter(issues::id.eq(issue.id)))
                .set(issues::num_comments.eq(issues::num_comments + 1))
                .execute(&conn.0)?;
            
            diesel::insert_into(comments::table)
                .values((
                    comments::enum_type.eq(0),
                    comments::issue_id.eq(issue.id),
                    comments::user_id.eq(new_comment.user_id),
                    comments::content.eq(&new_comment.content)
                ))
                .execute(&conn.0)?;

            let mut comment: models::Comment = comments::table
                .filter(comments::issue_id.eq(issue.id).and(comments::user_id.eq(new_comment.user_id)))
                .inner_join(users::table.inner_join(emails::table))
                .filter(emails::is_primary.eq(true))
                .order(comments::created_at.desc())
                .select((comments::all_columns, users::all_columns, emails::all_columns))
                .first::<(db::Comment, db::User, db::Email)>(&conn.0)
                .map(|(c, u, e)| (c, (u, e).into(), None).into())?;

            if let Some(attachment_ids) = &new_comment.attachment_ids {
                let attachments = attachment_ids.iter()
                    .flat_map(|a| db::Attachment::by_uuid(conn, &a).ok())
                    .collect::<Vec<_>>();
                comment.attachments = db::Attachment::insert_all_for_comment_id(conn, comment.id ,&attachments).ok();
            }

            Ok(comment)
        })
    }

    pub fn delete(conn: &DbConn, issue: &models::Issue) -> QueryResult<()> {
        conn.0.transaction::<_, diesel::result::Error, _>(|| {
            if issue.comments.is_some() {
                for comment in issue.comments.as_ref().unwrap() {
                    db::Comment::delete(conn, &comment)?;
                }
            }
            
            if issue.labels.is_some() {
                for label in issue.labels.as_ref().unwrap() {
                    db::Label::delete(conn, &label)?;
                }
            }

            if issue.assignees.is_some() {
                for assignee in issue.assignees.as_ref().unwrap() {
                    diesel::delete(issues_users::table.filter(issues_users::issue_id.eq(issue.id).and(issues_users::user_id.eq(assignee.id))))
                        .execute(&conn.0)
                        .map(|_| ())?;
                }
            }

            diesel::delete(issues::table.find(issue.id))
                .execute(&conn.0)
                .map(|_| ())
        })
    }

    pub fn delete_all_for_project(conn: &DbConn, project: &models::Project) -> QueryResult<()> {
        conn.0.transaction::<_, diesel::result::Error, _>(|| {
            let issues = db::Issue::all_for_project(conn, project)?;
            for issue in issues {
                db::Issue::delete(conn, &issue)?;
            }

            Ok(())
        })
    }
}
