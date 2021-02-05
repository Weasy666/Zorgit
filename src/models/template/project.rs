use std::borrow::Borrow;
use askama::Template;
use crate::models::{Comment, Issue, Label, PullRequest, Project, User, SourceEntry, template::Tab};
use crate::vcs::{Commit, Diff, DiffFile, SourceEntries};
use crate::utils::humanize::*;


#[derive(Template)]
#[template(path = "project/new.html")]
pub struct NewProjectTemplate {
    pub debug: bool,
    pub user: Option<User>,
    pub owners: Vec<User>,
}

#[derive(Template)]
#[template(path = "project/code/base.html")]
pub struct CodeTemplate { 
    pub debug: bool,
    pub user: Option<User>,
    pub project: Project,
    pub active_tab: Tab,
    pub branch: Option<String>,
    pub branches: Option<Vec<String>>,
    pub num_commits: usize,
    pub num_contributors: usize,
    pub license: Option<SourceEntry>,
    pub entries: Option<SourceEntries>,
    pub last_commit: Option<Commit>,
    pub readme: Option<SourceEntry>,
    pub is_file: bool,
}

#[derive(Template)]
#[template(path = "project/commit.html"/*, print = "code"*/)]
pub struct CommitTemplate {
    pub debug: bool,
    pub user: Option<User>,
    pub project: Project,
    pub active_tab: Tab,
    pub branch: Option<String>,
    pub commit: Commit,
    pub diff: Diff,
}

#[derive(Template)]
#[template(path = "project/commits.html")]
pub struct CommitsTemplate {
    pub debug: bool,
    pub user: Option<User>,
    pub project: Project,
    pub active_tab: Tab,
    pub branch: Option<String>,
    pub branches: Option<Vec<String>>,
    pub num_commits: usize,
    pub commits: Vec<Commit>,
}

#[derive(Template)]
#[template(path = "project/branches.html")]
pub struct BranchesTemplate {
    pub debug: bool,
    pub user: Option<User>,
    pub project: Project,
    pub active_tab: Tab,
    pub branch: Option<String>,
    pub branches: Option<Vec<(String, Commit)>>,
}

#[derive(Template)]
#[template(path = "project/issues/issues.html")]
pub struct IssuesTemplate {
    pub debug: bool,
    pub user: Option<User>,
    pub project: Project,
    pub active_tab: Tab,
    pub branch: Option<String>,
    pub issues: Option<Vec<Issue>>,
}

#[derive(Template)]
#[template(path = "project/issues/issue.html")]
pub struct IssueTemplate {
    pub debug: bool,
    pub user: Option<User>,
    pub project: Project,
    pub active_tab: Tab,
    pub branch: Option<String>,
    pub issue: Issue,
    pub active_labels: Option<Vec<i32>>,
    pub active_assignees: Option<Vec<i32>>,
    pub labels: Option<Vec<Label>>,
    pub collaborators: Vec<User>,
}

#[derive(Template)]
#[template(path = "project/issues/new.html")]
pub struct NewIssueTemplate {
    pub debug: bool,
    pub user: Option<User>,
    pub project: Project,
    pub active_tab: Tab,
    pub branch: Option<String>,
    pub active_labels: Option<Vec<i32>>,
    pub active_assignees: Option<Vec<i32>>,
    pub labels: Option<Vec<Label>>,
    pub collaborators: Vec<User>,
}

#[derive(Template)]
#[template(path = "project/issues/labels.html")]
pub struct LabelsTemplate {
    pub debug: bool,
    pub user: Option<User>,
    pub project: Project,
    pub active_tab: Tab,
    pub branch: Option<String>,
    pub labels: Option<Vec<Label>>,
}

#[derive(Template)]
#[template(path = "project/pull_request.html")]
pub struct PullTemplate {
    pub debug: bool,
    pub user: Option<User>,
    pub project: Project,
    pub active_tab: Tab,
    pub branch: Option<String>,
}

#[derive(Template)]
#[template(path = "project/pull_requests.html")]
pub struct PullsTemplate {
    pub debug: bool,
    pub user: Option<User>,
    pub project: Project,
    pub active_tab: Tab,
    pub branch: Option<String>,
    pub pulls: Vec<PullRequest>,
}

#[derive(Template)]
#[template(path = "project/releases.html")]
pub struct ReleasesTemplate {
    pub debug: bool,
    pub user: Option<User>,
    pub project: Project,
    pub active_tab: Tab,
    pub branch: Option<String>,
}

#[derive(Template)]
#[template(path = "project/wiki.html")]
pub struct WikiTemplate {
    pub debug: bool,
    pub user: Option<User>,
    pub project: Project,
    pub active_tab: Tab,
    pub branch: Option<String>,
}

#[derive(Template)]
#[template(path = "project/settings/project.html")]
pub struct SettingsProjectTemplate {
    pub debug: bool,
    pub user: Option<User>,
    pub project: Project,
    pub active_tab: Tab,
    pub branch: Option<String>,
    pub settings_tab: Tab,
}

#[derive(Template)]
#[template(path = "project/settings/collaboration.html")]
pub struct SettingsCollaborationTemplate {
    pub debug: bool,
    pub user: Option<User>,
    pub project: Project,
    pub active_tab: Tab,
    pub branch: Option<String>,
    pub settings_tab: Tab,
}

#[derive(Template)]
#[template(path = "project/settings/branches.html")]
pub struct SettingsBranchesTemplate {
    pub debug: bool,
    pub user: Option<User>,
    pub project: Project,
    pub active_tab: Tab,
    pub branch: Option<String>,
    pub settings_tab: Tab,
}

#[derive(Template)]
#[template(path = "project/watchers.html")]
pub struct WatchersTemplate {
    pub debug: bool,
    pub user: Option<User>,
    pub project: Project,
    pub active_tab: Tab,
    pub branch: Option<String>,
}

#[derive(Template)]
#[template(path = "project/stars.html")]
pub struct StarsTemplate {
    pub debug: bool,
    pub user: Option<User>,
    pub project: Project,
    pub active_tab: Tab,
    pub branch: Option<String>,
}

#[derive(Template)]
#[template(path = "project/forks.html")]
pub struct ForksTemplate {
    pub debug: bool,
    pub user: Option<User>,
    pub project: Project,
    pub active_tab: Tab,
    pub branch: Option<String>,
}