use crate::DbConn;
use crate::db;
use crate::models::{self, Email, Language, Url};
use diesel::prelude::*;
use chrono::{NaiveDateTime, Utc};
use crate::db::schema::*;
use std::path::PathBuf;

#[derive(Identifiable, Associations, Clone, Debug)]
//#[changeset_options(treat_none_as_null("true"))]
pub struct User {
    pub id: i32,
    pub types: i16,
    pub username: String,
    pub full_name: Option<String>,
    pub avatar: PathBuf,
    pub avatar_email: Option<Email>,
    pub location: Option<String>,
    pub website: Option<Url>,
    pub description: Option<String>,
    pub language: Language,

    pub must_change_password: bool,
    pub is_email_hidden: bool,
    pub is_admin: bool,
    pub is_organisation: bool,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub last_seen_at: NaiveDateTime,
}

impl User {
    pub fn new_user(conn: &DbConn, new_user: models::NewUser) -> QueryResult<models::User> {
        conn.0.transaction::<_, diesel::result::Error, _>(|| {
            diesel::insert_into(users::table)
                .values(&new_user)
                .execute(&conn.0)?;
            
            let user = users::table.filter(users::username.eq(&new_user.username))
                .first::<db::User>(&conn.0)?;
            let basic_user = models::User::Individual(models::UserInner {
                id: user.id,
                username: user.username.clone(),
                full_name: user.full_name.clone(),
                avatar: PathBuf::default(),
                avatar_email: None,
                email: "unknown@example.org".parse::<Email>().unwrap(),
                location: None,
                website: None,
                description: None,
                language: "en-EN".parse::<Language>().unwrap(),
    
                must_change_password: user.must_change_password,
                is_email_hidden: user.is_email_hidden,
                is_admin: user.is_admin,
                is_organisation: user.is_organisation,
                
                created_at: user.created_at,
                updated_at: user.updated_at,
                last_seen_at: user.last_seen_at,
            });
            let new_email = models::NewEmail {
                id: new_user.email.id,
                address: new_user.email.address,
                is_primary: new_user.email.is_primary,
                notification: new_user.email.notification,
            };
            
            db::User::new_email_for_user(conn, &basic_user, &new_email)
                .map(|e| (user, e).into())
        })
    }
    
    pub fn new_email(conn: &DbConn, user: &models::User, new_email: &models::NewEmail) -> QueryResult<models::Email> {
        db::User::new_email_for_user(conn, user, new_email)
    }

    pub fn delete_email(conn: &DbConn, user: &models::User, delete_email: &models::DeleteEmail) -> QueryResult<()> {
        diesel::delete(emails::table.filter(emails::user_id.eq(user.id).and(emails::id.eq(delete_email.id)).and(emails::is_primary.eq(false))))
            .execute(&conn.0)
            .map(|_| ())
    }

    pub fn all_emails(conn: &DbConn, user: &models::User) -> QueryResult<Vec<models::Email>> {
        emails::table.filter(emails::user_id.eq(user.id))
            .load::<db::Email>(&conn.0)
    }
    
    pub fn new_email_for_user(conn: &DbConn, user: &models::User, new_email: &models::NewEmail) -> QueryResult<models::Email> {
        db::Email::new_for_user(conn, user, new_email)
    }

    pub fn update_profile(conn: &DbConn, user: &models::User, update_profile: &models::UpdateProfile) -> QueryResult<()> {
        conn.0.transaction::<_, diesel::result::Error, _>(|| {
            diesel::update(users::table.find(user.id))
                .set((
                    update_profile.avatar.to_str().map(ToString::to_string).map(|a| users::avatar.eq(a)),
                    users::location.eq(&update_profile.location),
                    users::website.eq(update_profile.website.as_ref().map(|url| url.as_str())),
                    users::description.eq(&update_profile.description),
                    users::language.eq(&update_profile.language.to_string()),
                    users::is_email_hidden.eq(&update_profile.is_email_hidden),
                    users::updated_at.eq(Utc::now().naive_utc()),
                ))
                .execute(&conn.0)
                .map(|_| ())?;
            diesel::update(emails::table.find(&update_profile.email.id))
                .set(emails::is_primary.eq(&update_profile.email.is_primary))
                .execute(&conn.0)
                .map(|_| ())
        })
    }

    pub fn update_account(conn: &DbConn, user: &models::User, update_account: &models::UpdateAccount) -> QueryResult<()> {
        let pwd = update_account.password.password().unwrap();
        let salt = update_account.password.salt().unwrap();
        diesel::update(users::table.find(user.id))
            .set((
                users::username.eq(&update_account.username),
                users::password.eq(&pwd),
                users::salt.eq(&salt),
                users::updated_at.eq(Utc::now().naive_utc()),
            ))
            .execute(&conn.0)
            .map(|_| ())
    }

    pub fn update_password(conn: &DbConn, user: &models::User, update_password: &models::UpdatePassword) -> QueryResult<()> {
        let pwd = update_password.new.password().unwrap();
        let salt = update_password.new.salt().unwrap();
        diesel::update(users::table.find(user.id))
            .set((
                users::password.eq(&pwd),
                users::salt.eq(&salt),
                users::updated_at.eq(Utc::now().naive_utc()),
            ))
            .execute(&conn.0)
            .map(|_| ())
    }

    pub fn update_email(conn: &DbConn, user: &models::User, update_email: &models::UpdateEmail) -> QueryResult<()> {
        conn.0.transaction::<_, diesel::result::Error, _>(|| {
            if update_email.is_primary && update_email.id != user.email.id {
                diesel::update(emails::table.find(user.email.id))
                    .set(emails::is_primary.eq(false))
                    .execute(&conn.0)
                    .map(|_| ())?;
            }
            
            let notification: i16 = update_email.notification.clone().into();
            diesel::update(emails::table.find(update_email.id))
                .set((
                    emails::is_primary.eq(&update_email.is_primary),
                    emails::notification.eq(&notification),
                ))
                .execute(&conn.0)
                .map(|_| ())
        })
    }

    pub fn by_name(conn: &DbConn, username: &str) -> QueryResult<models::User> {
        users::table.filter(users::username.eq(username))
            .inner_join(emails::table)
            .first::<(db::User, db::Email)>(&conn.0)
            .map(|(u,e)| (u, e).into())
    }

    pub fn id_for_name(conn: &DbConn, username: &str) -> QueryResult<i32> {
        users::table.filter(users::username.eq(username))
            .select(users::id)
            .first(&conn.0)
    }

    pub fn by_name_or_email(conn: &DbConn, keyword: &str) -> QueryResult<models::User> {
        users::table.inner_join(emails::table)
            .filter(users::username.eq(keyword).or(emails::address.eq(keyword)))
            .select((users::all_columns, emails::all_columns))
            .first::<(db::User, db::Email)>(&conn.0)
            .map(|(u, e)| (u, e).into())
    }

    pub fn by_id(conn: &DbConn, user_id: i32) -> QueryResult<models::User> {
        users::table.find(user_id)
            .inner_join(emails::table)
            .first::<(db::User, db::Email)>(&conn.0)
            .map(|(u, e)| (u, e).into())
    }

    pub fn name_by_id(conn: &DbConn, user_id: i32) -> QueryResult<String> {
        users::table.find(user_id)
            .select(users::username)
            .first(&conn.0)
    }

    pub fn project_with_name_for_user(conn: &DbConn, projectname: &str, username: &str) -> QueryResult<models::Project> {
        db::Project::by_name_for_user(conn, projectname, username)
    }

    pub fn projects_for_user(conn: &DbConn, username: &str) -> QueryResult<Vec<models::Project>> {
        db::Project::all_for_username(conn, username)
    }

    pub fn all_for_issue_id(conn: &DbConn, issue_id: i32) -> QueryResult<Vec<models::User>> {        
        issues_users::table
            .filter(issues_users::issue_id.eq(issue_id))
            .inner_join(users::table.inner_join(emails::table))
            .select((users::all_columns, emails::all_columns))
            .load::<(db::User, db::Email)>(&conn.0)
            .map(|v| v.into_iter().map(|(u,e)| (u, e).into()).collect::<Vec<_>>())
    }

    pub fn update_assignees_for_issue_id(conn: &DbConn, issue_id: i32, assignees: &[models::User]) -> QueryResult<Vec<models::User>> {
        conn.0.transaction::<_, diesel::result::Error, _>(|| {
            let assignees_old = User::all_for_issue_id(conn, issue_id)?;

            for assignee in assignees_old {
                User::delete_assignee_for_issue_id(conn, issue_id, &assignee)?;
            }

            User::insert_all_assignees_for_issue_id(conn, issue_id, &assignees)
        })
    }

    pub fn delete_assignee_for_issue_id(conn: &DbConn, issue_id: i32, assignee: &models::User) -> QueryResult<()> {
        diesel::delete(issues_users::table.filter(issues_users::issue_id.eq(issue_id).and(issues_users::user_id.eq(assignee.id))))
            .execute(&conn.0)
            .map(|_| ())
    }

    pub fn all_for_project_id(conn: &DbConn, project_id: i32) -> QueryResult<Vec<models::User>> {        
        projects_users::table
            .filter(projects_users::project_id.eq(project_id))
            .inner_join(users::table.inner_join(emails::table))
            .select((users::all_columns, emails::all_columns))
            .load::<(db::User, db::Email)>(&conn.0)
            .map(|v| v.into_iter().map(|(u,e)| (u, e).into()).collect::<Vec<_>>())
    }

    pub fn all_organisations(_conn: &DbConn, user: &models::User) -> QueryResult<Vec<models::User>> {
        // TODO: when organisations are implemented, load all organisations the user is a part of and has sufficent permissions.
        Ok([user.clone()].to_vec())
    }

    pub fn all_collaborations(conn: &DbConn, user: &models::User) -> QueryResult<Vec<models::Project>> {
        projects_users::table
            .filter(projects_users::user_id.eq(user.id))
            .inner_join(projects::table)
            .inner_join(users::table.inner_join(emails::table))
            .filter(emails::is_primary.eq(true))
            .select((users::all_columns, emails::all_columns, projects::all_columns))
            .load::<(db::User, db::Email, db::Project)>(&conn.0)
            .map(|v| 
                v.into_iter()
                    .filter(|(u, _, p)| p.user_id == u.id)
                    .map(|(u, e, p)| (p, (u, e).into(), None).into()).collect::<Vec<models::Project>>()
            )
    }

    pub fn insert_all_assignees_for_issue_id(conn: &DbConn, issue_id: i32, assignees: &[models::User]) -> QueryResult<Vec<models::User>> {
        conn.0.transaction::<_, diesel::result::Error, _>(|| {
            let assignees = assignees.iter()
                .flat_map(|a| User::insert_assignee_by_id_for_issue_id(conn, issue_id, a.id)) //TODO: use bulk insert
                .collect::<Vec<_>>();

            Ok(assignees)
        })
    }

    pub fn insert_assignee_by_id_for_issue_id(conn: &DbConn, issue_id: i32, assignee_id: i32)  -> QueryResult<models::User> {
        conn.0.transaction::<_, diesel::result::Error, _>(|| {
            diesel::insert_into(issues_users::table)
                .values((
                    issues_users::user_id.eq(assignee_id),
                    issues_users::issue_id.eq(issue_id)
                ))
                .execute(&conn.0)?;
            users::table.filter(users::id.eq(assignee_id))
                .inner_join(emails::table)
                .first::<(db::User, db::Email)>(&conn.0)
                .map(|(u, e)| (u, e).into())
        })
    }

    pub fn get_or_find_project(conn:&DbConn, user: &models::User, projectname: &str) -> QueryResult<models::Project> {
        let mut project = db::User::project_with_name_for_user(conn, projectname, &user.username);

        if project.is_err() {
            let projects = db::Project::find_and_init_for_user(&conn, user);
            if projects.is_empty() {
                return Err(diesel::result::Error::NotFound);
            }
            project = projects.into_iter()
                .find(|p| p.name == projectname)
                .ok_or(diesel::result::Error::NotFound);
        }
        project
    }
}

impl Queryable<users::SqlType, diesel::sqlite::Sqlite> for User {
    type Row = (i32,i16,String,Option<String>,String,Option<String>,String,String,Option<String>,Option<String>,Option<String>,
                String,bool,bool,bool,bool,NaiveDateTime,NaiveDateTime,NaiveDateTime);

    fn build((id, types, username, full_name, avatar, avatar_email, _password, _salt, location, website, description, language,
            must_change_password, is_email_hidden, is_admin, is_organisation, created_at, updated_at, last_seen_at): Self::Row
    ) -> Self {
        Self {
            id,
            types,
            username,
            full_name,
            avatar: PathBuf::from(avatar),
            avatar_email: avatar_email.map(|a| a.parse::<Email>().expect(&format!("Not a valid email address: {}", a))),
            location,
            website: website.map(|w| w.parse::<Url>().expect(&format!("Not a valid Url: {}", w))),
            description,
            language: language.parse::<Language>().expect(&format!("Not a valid Language Identifier: {}", language)),
            must_change_password,
            is_email_hidden,
            is_admin,
            is_organisation,
            created_at,
            updated_at,
            last_seen_at,
        }
    }
}
