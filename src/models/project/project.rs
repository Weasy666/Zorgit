use crate::DbConn;
use crate::db;
use crate::models::{Config, BasicSettings, Comment, NewComment, Issue, NewIssue, UpdateIssue, Topic, User, Label, NewLabel, Url};
use crate::vcs::{SourceEntries, VCS, Server, VersionControl, Commit, Diff, git};
use crate::utils::IntoOption;
use chrono::prelude::*;
use std::path::{Path, PathBuf};
use diesel::Connection;
use remove_dir_all;
use anyhow::Result;

pub struct Project {
    pub id: i32,
    pub owner: User,
    pub name: String,
    pub description: Option<String>,
    pub website: Option<Url>,
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

    pub is_fork: bool,
    pub forked_project: Option<i32>,
    pub disk_size: usize,
    pub topics: Option<Vec<Topic>>,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,

    pub vcs: VCS,
}

impl Project {
//##### Version Control operations #####
    pub fn init(&self, conn: &DbConn) -> Result<()> {
        self.vcs.init(Config::global().project_temp_dir(&self.name), &self.owner)?;
        let result = db::Project::init(conn, &self.name, self.owner.id)?;
        Ok(result)
    }

    pub fn default_branch(&mut self, conn: &DbConn) -> Result<Option<String>> {
        if self.default_branch.is_none() && !self.is_empty {
            self.default_branch = self.vcs.default_branch()?;
            if self.default_branch.is_some() {
                db::Project::set_default_branch(conn, self.id, &self.default_branch.as_ref().unwrap())?;
            }
        }
        Ok(self.default_branch.clone())
    }

    pub fn branches(&self) -> Result<Option<Vec<String>>> {
        self.vcs.branches()
    }

    pub fn branch_entries(&self, conn: &DbConn, branch_name: &str) -> Result<Option<SourceEntries>> {
        self.vcs.branch_entries(conn, branch_name)
    }

    pub fn branch_entry_by_path<P: AsRef<Path>>(&self, conn: &DbConn, branch_name: &str, path: P) -> Result<Option<SourceEntries>> {
        self.vcs.branch_entry_by_path(conn, branch_name, path)
    }

    pub fn raw_branch_entry_by_path<P: AsRef<Path>>(&self, branch_name: &str, path: P) -> Result<Option<Vec<u8>>> {
        self.vcs.raw_branch_entry_by_path(branch_name, path)
    }

    pub fn branch_last_commit(&self, conn: &DbConn, branch_name: &str) -> Result<Commit> {
        self.vcs.branch_last_commit(conn, branch_name)
    }

    pub fn branch_history(&self, conn: &DbConn, branch_name: &str, with_merge_commits: bool) -> Result<Vec<Commit>> {
        self.vcs.branch_history(conn, branch_name, with_merge_commits)
    }

    pub fn branch_commits_count(&self, branch_name: &str) -> Result<usize> {
        self.vcs.branch_commits_count(branch_name)
    }

    pub fn commit_ancestor_count(&self, commit_id: &str) -> Result<usize> {
        self.vcs.commit_ancestor_count(commit_id)
    }

    pub fn commit_associated_branches(&self, commit_id: &str) -> Result<Vec<String>> {
        self.vcs.commit_associated_branches(commit_id)
    }

    pub fn commit_by_id(&self, conn: &DbConn, commit_id: &str) -> Result<Commit> {
        self.vcs.commit_by_id(conn, commit_id)
    }

    pub fn diff_to_parent(&self, conn: &DbConn, commit: &Commit) -> Result<Diff> {
        self.vcs.diff_to_parent(conn, commit)
    }


//##### Database operations #####
    pub fn add_topics_by_id(&self, conn: &DbConn, topics: &[i32]) -> Result<Vec<Topic>> {
        let topics = db::Project::add_topics_by_id(conn, self, topics)?;
        Ok(topics)
    }

    pub fn all_issues(&self, conn: &DbConn) -> Result<Option<Vec<Issue>>> {
        let issues = db::Project::all_issues(conn, self)?;
        Ok(issues.into_option())
    }

    pub fn issue_by_number(&self, conn: &DbConn, issue_num: i32) -> Result<Issue> {
        let issue = db::Project::issue_by_number(conn, self, issue_num)?;
        Ok(issue)
    }

    pub fn new_issue(&self, conn: &DbConn, new_issue: &NewIssue) -> Result<Issue> {
        let issue = db::Project::new_issue(conn, self, new_issue)?;
        Ok(issue)
    }

    pub fn update_issue_number(&self, conn: &DbConn, issue_num: i32, update_issue: &UpdateIssue) -> Result<Issue> {
        let issue = db::Project::update_issue_number(conn, self, issue_num, update_issue)?;
        Ok(issue)
    }

    pub fn new_comment_for_issue_number(&self, conn: &DbConn, issue_num: i32, new_comment: &NewComment) -> Result<Comment> {
        let comment = db::Project::new_comment_for_issue_number(conn, self, issue_num, new_comment)?;
        Ok(comment)
    }

    pub fn all_labels(&self, conn: &DbConn) -> Result<Option<Vec<Label>>> {
        let labels = db::Project::all_labels(conn, self)?;
        Ok(labels.into_option())
    }

    pub fn new_label(&self, conn: &DbConn, new_label: &NewLabel) -> Result<Label> {
        let label = db::Project::new_label(conn, self, new_label)?;
        Ok(label)
    }

    pub fn all_collaborators(&self, conn: &DbConn) -> Result<Option<Vec<User>>> {
        let collaborators = db::Project::all_collaborators(conn, self)?;
        Ok(collaborators.into_option())
    }

    pub fn new_collaborator(&self, conn: &DbConn, new_collaborator: &User) -> Result<()> {
        let result = db::Project::new_collaborator(conn, self, new_collaborator)?;
        Ok(result)
    }

    pub fn update_basic_setting(&self, conn: &DbConn, settings: &BasicSettings) -> Result<()> {
        db::Project::update_basic_setting(conn, self, settings)?;

        if self.name != settings.project_name {
            // rename project directory
            let old_dir = self.dir();
            let new_dir = Project::compose_dir(&self.owner, &settings.project_name);
            std::fs::rename(old_dir, new_dir)?;
        }
        
        Ok(())
    }

    pub fn transfer_ownership(&self, conn: &DbConn, new_owner: &User) -> Result<()> {
        db::Project::transfer_ownership(conn, self, new_owner)?;
        
        // rename/move project directory
        let old_dir = self.dir();
        let new_dir = Project::compose_dir(new_owner, &self.name);
        std::fs::create_dir_all(new_owner.get_projects_dir())?;
        std::fs::rename(old_dir, new_dir)?;

        Ok(())
    }

    pub fn delete(&self, conn: &DbConn) -> Result<()> {
        conn.0.transaction::<_, anyhow::Error, _>(|| {
            db::Project::delete(conn, self)?;
            
            // remove project directory
            remove_dir_all::remove_dir_all(self.dir())?;

            Ok(())
        })
    }


//##### General functions #####
    pub fn url_relative(&self) -> String {
        self.url_absolute().path().to_owned()
    }

    pub fn url_absolute(&self) -> Url {
        Config::global().root_url().join(&[self.owner.username.as_str(), self.name.as_str()].join("/")).unwrap().into()
    }

    pub fn clone_url_http(&self) -> Url {
        Config::global().root_url().join(&format!("{}/{}.{}", self.owner.username, self.name, self.vcs.server().extension())).unwrap().into()
    }

    pub fn update_size(&mut self) -> Result<usize> {
        let size = self.vcs.calc_size()?;
        self.disk_size = size;
        Ok(size)
    }

    pub fn dir(&self) -> PathBuf {
        Project::compose_dir(&self.owner, &self.name)
    }

    pub fn attachments_dir(&self) -> PathBuf {
        Project::compose_attachments_dir(&self.owner.username, &self.name)
    }

    pub fn compose_dir(user: &User, projectname: &str) -> PathBuf {
        user.get_projects_dir().join(projectname)
    }

    pub fn compose_attachments_dir(username: &str, projectname: &str) -> PathBuf {
        Config::global().attachments_dir().join(username).join(projectname)
    }
}

impl From<(db::Project, User, Option<Vec<Topic>>)> for Project {
    fn from(origin: (db::Project, User, Option<Vec<Topic>>)) -> Project {
        let path = origin.1.get_projects_dir().join(&origin.0.name);
        Project {
            id: origin.0.id,
            owner: origin.1,
            name: origin.0.name.clone(),
            description: origin.0.description,
            website: origin.0.website.map(|w| w.parse::<Url>().expect("Invalid url")),
            default_branch: origin.0.default_branch,

            num_watches: origin.0.num_watches,
            num_stars: origin.0.num_stars,
            num_forks: origin.0.num_forks,
            num_issues: origin.0.num_issues,
            num_issues_closed: origin.0.num_issues_closed,
            num_issues_open: origin.0.num_issues_open,
            num_labels: origin.0.num_labels,
            num_pull_reqs: origin.0.num_pull_reqs,
            num_pull_reqs_closed: origin.0.num_pull_reqs_closed,
            num_pull_reqs_open: origin.0.num_pull_reqs_open,
            num_milestones: origin.0.num_milestones,
            num_milestones_closed: origin.0.num_milestones_closed,
            num_milestones_open: origin.0.num_milestones_open,
            num_releases: origin.0.num_releases,

            is_private: origin.0.is_private,
            is_empty: origin.0.is_empty,
            is_archived: origin.0.is_archived,

            is_fork: origin.0.is_fork,
            forked_project: origin.0.forked_project,
            disk_size: origin.0.disk_size as usize,
            topics: origin.2,

            created_at: origin.0.created_at,
            updated_at: origin.0.updated_at,

            vcs: match origin.0.vcs {
                0 => {
                    VCS::Git(
                        git::Repository::open(&path).expect(&format!("No git repository found at location: {:?}", &path))
                    )
                },
                n => panic!("unknown vcs: {}", n),
            },
        }
    }
}