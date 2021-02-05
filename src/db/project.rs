use crate::DbConn;
use diesel::prelude::*;
use crate::db;
use crate::models;
use crate::utils::IntoOption;
use crate::db::schema::*;
use chrono::prelude::*;
use std::path::Path;
use std::str;
use anyhow::Result;

#[derive(Identifiable, Associations, Queryable)]
#[table_name="projects"]
#[belongs_to(db::User)]
pub struct Project {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub website: Option<String>,
    pub default_branch: Option<String>,

    pub num_watches: i32,
    pub num_stars: i32,
    pub num_forks: i32,
    pub num_issues: i32,
    pub num_issues_closed: i32,
    pub num_issues_open: i32,
    pub num_labels: i32,
    pub num_pull_reqs: i32,
    pub num_pull_reqs_closed: i32,
    pub num_pull_reqs_open: i32,
    pub num_milestones: i32,
    pub num_milestones_closed: i32,
    pub num_milestones_open: i32,
    pub num_releases: i32,

    pub is_private: bool,
    /// Indicates whether the VCS(Version Control System) is initialized. Meaning if there is already any code committed to the system.
    pub is_empty: bool,
    pub is_archived: bool,

    pub vcs: i32,
    pub is_fork: bool,
    pub forked_project: Option<i32>,
    pub disk_size: i64,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Project {
    pub fn new(conn: &DbConn, new_project: &models::NewProject) -> QueryResult<models::Project> {
        conn.0.transaction::<_, diesel::result::Error, _>(|| {
            diesel::insert_into(projects::table)
                .values(new_project)
                .execute(&conn.0)?;

            let project = db::Project::by_name_for_user(conn, &new_project.name, &new_project.ownername)?;
            diesel::insert_into(projects_users::table)
                .values((projects_users::project_id.eq(project.id), projects_users::user_id.eq(new_project.user_id)))
                .execute(&conn.0)?;

            Ok(project)
        })
    }

    pub fn init(conn: &DbConn, projectname: &str, user_id: i32) -> QueryResult<()> {
        diesel::update(projects::table.filter(projects::name.eq(projectname).and(projects::user_id.eq(user_id))))
            .set(projects::is_empty.eq(false))
            .execute(&conn.0)
            .map(|_| ())
    }

    pub fn set_default_branch(conn: &DbConn, project_id: i32, default_branch: &str) -> QueryResult<()> {
        diesel::update(projects::table.filter(projects::id.eq(project_id)))
            .set(projects::default_branch.eq(default_branch))
            .execute(&conn.0)
            .map(|_| ())
    }

    pub fn by_name_for_user(conn: &DbConn, projectname: &str, username: &str) -> QueryResult<models::Project> {
        let mut project: models::Project = users::table.inner_join(emails::table)
            .filter(users::username.eq(username))
            .inner_join(projects::table)
            .filter(projects::name.eq(projectname))
            .select((projects::all_columns, users::all_columns, emails::all_columns))
            .first::<(db::Project, db::User, db::Email)>(&conn.0)
            .map(|(p,u,e)| (p, (u, e).into(), None).into())?;

        let topics = project_topics::table.filter(project_topics::project_id.eq(project.id))
            .inner_join(topics::table)
            .select(topics::all_columns)
            .load::<db::Topic>(&conn.0)
            .map(|v| v.into_iter().map(|t| t.into()).collect::<Vec<_>>())?;
        project.topics = topics.into_option();

        Ok(project)
    }

    pub fn by_id(conn: &DbConn, project_id: i32) -> QueryResult<models::Project> {
        projects::table
            .find(project_id)
            .inner_join(users::table.inner_join(emails::table))
            .filter(emails::is_primary.eq(true))
            .select((projects::all_columns, users::all_columns, emails::all_columns))
            .first::<(db::Project, db::User, db::Email)>(&conn.0)
            .map(|(p,u,e)| (p, (u, e).into(), None).into())
    }

    pub fn all_for_user(conn: &DbConn, user: &models::User) -> QueryResult<Vec<models::Project>> {
        projects::table.filter(projects::user_id.eq(user.id))
            .load::<db::Project>(&conn.0)
            .map(|v| v.into_iter().map(|p| (p, user.clone(), None).into()).collect::<Vec<_>>())
    }

    pub fn all_for_username(conn: &DbConn, username: &str) -> QueryResult<Vec<models::Project>> {
        projects::table.inner_join(users::table.inner_join(emails::table))
            .filter(users::username.eq(username).and(emails::is_primary.eq(true)))
            .select((projects::all_columns, users::all_columns, emails::all_columns))
            .load::<(db::Project, db::User, db::Email)>(&conn.0)
            .map(|v| v.into_iter().map(|(p, u, e)| (p, (u, e).into(), None).into()).collect::<Vec<_>>())
    }

    pub fn add_topics_by_id(conn: &DbConn, project: &models::Project, topic_ids: &[i32]) -> QueryResult<Vec<models::Topic>> {
        conn.0.transaction::<_, diesel::result::Error, _>(|| {
            let current_topics = project_topics::table.filter(project_topics::project_id.eq(project.id))
                .inner_join(topics::table)
                .select(topics::all_columns)
                .load::<db::Topic>(&conn.0)?;

            let mut topic_ids = topic_ids.to_vec();
            for topic in current_topics {
                if topic_ids.contains(&topic.id) {
                    topic_ids.iter().position(|&n| n == topic.id).map(|i| topic_ids.remove(i));
                }
                else {
                    db::Project::remove_topic_by_id(conn, project, topic.id)?;
                }
            }

            topic_ids.into_iter()
                .map(|t_id| db::Project::add_topic_by_id(conn, project, t_id))
                .collect::<QueryResult<Vec<_>>>()
        })
    }

    pub fn add_topic_by_id(conn: &DbConn, project: &models::Project, topic_id: i32) -> QueryResult<models::Topic> {
        diesel::insert_into(project_topics::table)
            .values((project_topics::project_id.eq(project.id), project_topics::topic_id.eq(topic_id)))
            .execute(&conn.0)?;
        db::Topic::by_id(conn, topic_id)
    }

    pub fn remove_topic_by_id(conn: &DbConn, project: &models::Project, topic_id: i32) -> QueryResult<()> {
        diesel::delete(project_topics::table.filter(project_topics::project_id.eq(project.id).and(project_topics::topic_id.eq(topic_id))))
            .execute(&conn.0)
            .map(|_| ())
    }

    pub fn all_issues(conn: &DbConn, project: &models::Project) -> QueryResult<Vec<models::Issue>> {
        db::Issue::all_for_project(conn, project)
    }

    pub fn issue_by_number(conn: &DbConn, project: &models::Project, issue_num: i32) -> QueryResult<models::Issue> {
        db::Issue::for_project_by_number(conn, project, issue_num)
    }

    pub fn new_issue(conn: &DbConn, project: &models::Project, new_issue: &models::NewIssue) -> QueryResult<models::Issue> {
        conn.0.transaction::<_, diesel::result::Error, _>(|| {
            diesel::update(projects::table.filter(projects::id.eq(project.id)))
                .set((
                    projects::num_issues.eq(projects::num_issues + 1),
                    projects::num_issues_open.eq(projects::num_issues_open + 1)
                ))
                .execute(&conn.0)?;
            let issue_num = projects::table.filter(projects::id.eq(project.id))
                                .select(projects::num_issues)
                                .first::<i32>(&conn.0)?;

            diesel::insert_into(issues::table)
                .values((
                    issues::number.eq(issue_num),
                    issues::project_id.eq(project.id),
                    issues::user_id.eq(new_issue.user_id),
                    issues::title.eq(&new_issue.title),
                    issues::content.eq(&new_issue.content)
                ))
                .execute(&conn.0)?;

            let mut issue: models::Issue = issues::table
                .filter(issues::project_id.eq(project.id).and(issues::user_id.eq(new_issue.user_id)).and(issues::number.eq(issue_num)))
                .order(issues::created_at.desc())
                .inner_join(users::table.inner_join(emails::table))
                .filter(emails::is_primary.eq(true))
                .select((issues::all_columns, users::all_columns, emails::all_columns))
                .first::<(db::Issue, db::User, db::Email)>(&conn.0)
                .map(|(i, u, e)| (conn, i, (u, e).into()).into())?;

            if !new_issue.label_ids.is_empty() {
                let labels = db::Label::all_for_project_id(conn, project.id)?
                    .into_iter().filter(|l| new_issue.label_ids.contains(&l.id))
                    .collect::<Vec<_>>();
                issue.labels = db::Label::insert_all_for_issue_id(conn, issue.id, &labels)?.into();
            }

            if !new_issue.assignee_ids.is_empty() {
                let assignees = db::User::all_for_project_id(conn, project.id)?
                    .into_iter().filter(|a| new_issue.assignee_ids.contains(&a.id))
                    .collect::<Vec<_>>();
                issue.assignees = db::User::insert_all_assignees_for_issue_id(conn, issue.id, &assignees)?.into();
            }

            Ok(issue)
        })
    }

    pub fn update_issue_number(conn: &DbConn, project: &models::Project, issue_num: i32, update_issue: &models::UpdateIssue) -> QueryResult<models::Issue> {
        conn.0.transaction::<_, diesel::result::Error, _>(|| {
            let issue_old = Project::issue_by_number(conn, project, issue_num)?;
            let updated_at = Utc::now().naive_utc();
            diesel::update(issues::table.filter(issues::id.eq(issue_old.id)))
                .set((
                    issues::title.eq(&update_issue.title),
                    issues::content.eq(&update_issue.content),
                    issues::updated_at.eq(updated_at),
                ))
                .execute(&conn.0)?;

            let mut labels = Vec::new();
            if !update_issue.label_ids.is_empty() {
                labels = db::Label::all_for_project_id(conn, project.id)?
                    .into_iter().filter(|l| update_issue.label_ids.contains(&l.id))
                    .collect::<Vec<_>>();
            }
            db::Label::update_all_for_issue_id(conn, issue_old.id, &labels)?;

            let mut assignees = Vec::new();
            if !update_issue.assignee_ids.is_empty() {
                assignees = db::User::all_for_project_id(conn, project.id)?
                    .into_iter().filter(|a| update_issue.assignee_ids.contains(&a.id))
                    .collect::<Vec<_>>();
            }
            db::User::update_assignees_for_issue_id(conn, issue_old.id, &assignees)?;

            Project::issue_by_number(conn, project, issue_num)
        })
    }

    pub fn new_comment_for_issue_number(conn: &DbConn, project: &models::Project, issue_num: i32, new_comment: &models::NewComment) -> QueryResult<models::Comment> {
        let issue = Project::issue_by_number(conn, project, issue_num)?;
        issue.new_comment(conn, new_comment)
            .map_err(|e| e.downcast::<diesel::result::Error>().expect("Could not downcast to diesel::Error"))
    }

    pub fn all_labels(conn: &DbConn, project: &models::Project) -> QueryResult<Vec<models::Label>> {
        db::Label::all_for_project(conn, project)
    }

    pub fn new_label(conn: &DbConn, project: &models::Project, new_label: &models::NewLabel) -> QueryResult<models::Label> {
        db::Label::insert_for_project(conn, project, new_label)
    }

    pub fn all_collaborators(conn: &DbConn, project: &models::Project) -> QueryResult<Vec<models::User>> {
        projects_users::table.inner_join(users::table.inner_join(emails::table))
            .filter(projects_users::project_id.eq(project.id).and(emails::is_primary.eq(true)))
            .select((users::all_columns, emails::all_columns))
            .load::<(db::User, db::Email)>(&conn.0)
            .map(|v| v.into_iter().map(|(u,e)| (u, e).into()).collect::<Vec<_>>())
    }

    pub fn new_collaborator(conn: &DbConn, project: &models::Project, new_collaborator: &models::User) -> QueryResult<()> {
        diesel::insert_into(projects_users::table)
            .values((projects_users::project_id.eq(project.id), projects_users::user_id.eq(new_collaborator.id)))
            .execute(&conn.0)
            .map(|_| ())
    }

    /// Result = (Name, OwnerName)
    pub fn name_and_ownername_by_id(conn: &DbConn, project_id: i32) -> QueryResult<(String, String)> {
        projects::table.find(project_id)
            .inner_join(users::table)
            .select((projects::name, users::username))
            .first::<(String, String)>(&conn.0)
    }

    pub fn update_basic_setting(conn: &DbConn, project: &models::Project, settings: &models::BasicSettings) -> QueryResult<()> {
        conn.0.transaction::<_, diesel::result::Error, _>(|| {
            diesel::update(projects::table.filter(projects::id.eq(project.id)))
                .set((
                    projects::name.eq(&settings.project_name),
                    projects::is_private.eq(&settings.is_private),
                    projects::description.eq(&settings.description),
                    projects::website.eq(settings.website.as_ref().map(|url| url.as_str())),
                    projects::updated_at.eq(Utc::now().naive_utc()),
                ))
                .execute(&conn.0)
                .map(|_| ())?;

            if settings.topics.is_some() {
                db::Project::add_topics_by_id(conn, project, settings.topics.as_ref().unwrap().as_slice())?;
            }

            Ok(())
        })
    }

    pub fn transfer_ownership(conn: &DbConn, project: &models::Project, new_owner: &models::User) -> QueryResult<()> {
        conn.0.transaction::<_, diesel::result::Error, _>(|| {
            let exists = diesel::dsl::select(diesel::dsl::exists(projects_users::table.filter(projects_users::user_id.eq(new_owner.id).and(projects_users::project_id.eq(project.id)))))
                .get_result::<bool>(&conn.0)?;

            if !exists {
                diesel::insert_into(projects_users::table)
                    .values((projects_users::project_id.eq(project.id),projects_users::user_id.eq(new_owner.id)))
                    .execute(&conn.0)?;
            }

            diesel::update(projects::table.find(project.id))
                .set(projects::user_id.eq(&new_owner.id))
                .execute(&conn.0)
                .map(|_| ())
        })
    }

    pub fn delete(conn: &DbConn, project: &models::Project) -> QueryResult<()> {
        conn.0.transaction::<_, diesel::result::Error, _>(|| {
            // delete collaborators
            diesel::delete(projects_users::table.filter(projects_users::project_id.eq(project.id)))
                .execute(&conn.0)?;
            // delete topics
            diesel::delete(project_topics::table.filter(project_topics::project_id.eq(project.id)))
                .execute(&conn.0)?;
            db::Issue::delete_all_for_project(conn, project)?;
            diesel::delete(projects::table.find(project.id))
                .execute(&conn.0)
                .map(|_| ())
        })
    }



    /// Currently this is only for development purposes and only works with git projects, i'm not sure if this should be left for production.
    pub fn find_and_init_for_user(conn: &DbConn, user: &models::User) -> Vec<models::Project> {
        let path = user.get_projects_dir();
        if !path.is_dir() {
            return Vec::with_capacity(0);
        }

        let dir_walker = walkdir::WalkDir::new(path).min_depth(1).max_depth(1).into_iter();
        dir_walker.filter_entry(|e| !is_hidden(e) && is_dir(e))
            .filter_map(Result::ok)
            .map(|entry| git2::Repository::open(entry.path()))
            .filter_map(Result::ok)
            .map(|repo| {
                let name = extract_projectname(repo.path());
                let default_branch = repo.head().map(|h| h.shorthand().map(ToString::to_string)).unwrap_or(None);
                let new_project = models::NewProject {
                        user_id: user.id,
                        ownername: user.username.clone(),
                        name,
                        description: None,
                        website: None,
                        default_branch,
                        is_private: true,
                        is_empty: repo.is_empty().unwrap_or(true),
                        is_fork: false,
                        forked_project: None,
                        disk_size: usize::default(),
                        vcs: 0,
                    };
                db::Project::new(conn, &new_project)
            })
            .filter_map(Result::ok)
            .collect()
    }
}


fn is_hidden(entry: &walkdir::DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

fn is_dir(entry: &walkdir::DirEntry) -> bool {
    entry.metadata()
        .map(|m| m.is_dir())
        .unwrap_or(false)
}

fn extract_projectname(path: &Path) -> String {
    let mut projectname = path;
    if projectname.ends_with(".git") {
        projectname = path.parent().unwrap();
    }
    projectname.file_name().unwrap().to_str().unwrap().to_string()
}
