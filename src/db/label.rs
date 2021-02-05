use crate::DbConn;
use diesel::prelude::*;
use crate::db;
use crate::db::schema::*;
use crate::models::{self};

#[derive(Queryable, Identifiable, Associations, AsChangeset, Clone)]
#[belongs_to(db::Project)]
pub struct Label {
    pub id: i32,
    pub project_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub color: String
}

impl Label {
    pub fn insert_all_for_issue_id(conn: &DbConn, issue_id: i32, labels: &[models::Label]) -> QueryResult<Vec<models::Label>> {
        Ok(labels.iter()
            .flat_map(|l| Label::insert_for_issue_id(conn, l, issue_id))
            .collect::<Vec<_>>())
    }

    pub fn insert_for_issue_id(conn: &DbConn, label: &models::Label, issue_id: i32)  -> QueryResult<models::Label> {
        diesel::insert_into(issues_labels::table)
            .values((issues_labels::label_id.eq(label.id), issues_labels::issue_id.eq(issue_id)))
            .execute(&conn.0)?;

        Ok(models::Label {
            id: label.id,
            project_id: label.project_id,
            name: label.name.clone(),
            description: label.description.clone(),
            color: label.color.clone(),
        })
    }

    pub fn all_for_issue_id(conn: &DbConn, issue_id: i32)  -> QueryResult<Vec<models::Label>> {
        issues_labels::table.inner_join(labels::table)
            .filter(issues_labels::issue_id.eq(issue_id))
            .select((labels::id, labels::project_id, labels::name, labels::description, labels::color))
            .load::<Label>(&conn.0)
            .map(|v| v.into_iter().map(|l| l.into()).collect::<Vec<_>>())
    }

    pub fn update_all_for_issue_id(conn: &DbConn, issue_id: i32, labels: &[models::Label]) -> QueryResult<Vec<models::Label>> {
        conn.0.transaction::<_, diesel::result::Error, _>(|| {
            let labels_old = Label::all_for_issue_id(conn, issue_id)?;

            labels_old.iter()
                .map(|l| Label::delete_for_issue_id(conn, issue_id, l))
                .collect::<QueryResult<()>>()?;

            Label::insert_all_for_issue_id(conn, issue_id, labels)
        })
    }

    pub fn delete_for_issue_id(conn: &DbConn, issue_id: i32, label: &models::Label) -> QueryResult<()> {
        diesel::delete(issues_labels::table.filter(issues_labels::issue_id.eq(issue_id).and(issues_labels::label_id.eq(label.id))))
            .execute(&conn.0)
            .map(|_| ())
    }

    pub fn update(conn: &DbConn, update_label: &models::UpdateLabel) -> QueryResult<()> {
        diesel::update(labels::table.filter(labels::id.eq(update_label.id)))
            .set((
                labels::name.eq(&update_label.name),
                labels::description.eq(&update_label.description),
                labels::color.eq(&update_label.color),
            ))
            .execute(&conn.0)
            .map(|_| ())
    }

    pub fn delete(conn: &DbConn, delete_label: &models::DeleteLabel) -> QueryResult<()> {
        conn.0.transaction::<_, diesel::result::Error, _>(|| {
            diesel::delete(issues_labels::table.filter(issues_labels::label_id.eq(delete_label.id)))
                .execute(&conn.0)?;
            diesel::delete(labels::table.find(delete_label.id))
                .execute(&conn.0)?;

            diesel::update(projects::table.filter(projects::id.eq(delete_label.project_id)))
                .set(projects::num_labels.eq(projects::num_labels - 1))
                .execute(&conn.0)
                .map(|_| ())
        })
    }

    pub fn all_for_project(conn: &DbConn, project: &models::Project) -> QueryResult<Vec<models::Label>> {
        Label::all_for_project_id(conn, project.id)
    }

    pub fn all_for_project_id(conn: &DbConn, project_id: i32) -> QueryResult<Vec<models::Label>> {
        labels::table
            .filter(labels::project_id.eq(project_id))
            .select((labels::id, labels::project_id, labels::name, labels::description, labels::color))
            .load::<Label>(&conn.0)
            .map(|v| v.into_iter().map(|l| l.into()).collect::<Vec<_>>())
    }

    pub fn insert_for_project(conn: &DbConn, project: &models::Project, new_label: &models::NewLabel)  -> QueryResult<models::Label> {
        conn.0.transaction::<_, diesel::result::Error, _>(|| {
            diesel::update(projects::table.filter(projects::id.eq(project.id)))
                .set(projects::num_labels.eq(projects::num_labels + 1))
                .execute(&conn.0)?;

            diesel::insert_into(labels::table)
                .values((
                    labels::project_id.eq(project.id),
                    labels::name.eq(&new_label.name),
                    labels::description.eq(&new_label.description),
                    labels::color.eq(&new_label.color),
                ))
                .execute(&conn.0)?;

            labels::table
                .filter(labels::project_id.eq(project.id).and(labels::name.eq(&new_label.name)))
                .first::<Label>(&conn.0)
                .map(|l| l.into())
        })
    }
}