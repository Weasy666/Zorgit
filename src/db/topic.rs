use crate::models;
use crate::DbConn;
use crate::db;
use crate::db::schema::*;
use diesel::prelude::*;

#[derive(Queryable, Clone)]
pub struct Topic {
    pub id: i32,
    pub name: String,
}

impl Topic {
    pub fn new(conn: &DbConn, topic_name: &str) -> QueryResult<models::Topic> {
        diesel::insert_into(topics::table)
            .values(topics::name.eq(topic_name))
            .execute(&conn.0)?;

        topics::table.filter(topics::name.eq(topic_name))
            .first::<db::Topic>(&conn.0)
            .map(|t| t.into())
    }

    pub fn new_vec(conn: &DbConn, topic_name: &[String]) -> QueryResult<Vec<models::Topic>> {
        conn.0.transaction::<_, diesel::result::Error, _>(|| {
            topic_name.into_iter()
                .map(|t_name| db::Topic::new(conn, t_name))
                .collect::<QueryResult<Vec<_>>>()
        })
    }

    pub fn by_id(conn: &DbConn, topic_id: i32) -> QueryResult<models::Topic> {
        topics::table.find(topic_id)
                .select(topics::all_columns)
                .first::<db::Topic>(&conn.0)
                .map(|t| t.into())
    }

    pub fn matches(conn: &DbConn, query: &str) -> QueryResult<Vec<models::Topic>> {
        topics::table.filter(topics::name.like(format!("%{}%", query)))
            .load::<db::Topic>(&conn.0)
            .map(|v| v.into_iter().map(|t| t.into()).collect::<Vec<_>>())
    }
}