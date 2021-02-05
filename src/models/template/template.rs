use std::collections::{BTreeMap, HashMap};
use std::borrow::Borrow;
use askama::Template;
use crate::models::{Issue, Project, User};
use crate::utils::humanize::*;



#[derive(Template, Debug)]
#[template(path = "home.html")]
pub struct IndexTemplate {
    pub debug: bool,
    pub user: Option<User>,
}

#[derive(Template)]
#[template(path = "dashboard.html")]
pub struct DashboardTemplate {
    pub debug: bool,
    pub user: Option<User>,
    pub contexts: Vec<User>,
    pub projects: BTreeMap<i32, Vec<Project>>,
    pub num_orgs: i32,
}

#[derive(Template)]
#[template(path = "dashboard_issues.html")]
pub struct DashboardIssuesTemplate {
    pub debug: bool,
    pub user: Option<User>,
    pub num_issues_open: i32,
    pub num_issues_closed: i32,
    pub issues: Vec<Issue>,
    pub projects: HashMap<i32,Project>,
}

#[derive(Template)]
#[template(path = "dashboard_pull_requests.html")]
pub struct DashboardPullsTemplate {
    pub debug: bool,
    pub user: Option<User>,
}

#[derive(Template)]
#[template(path = "explore.html")]
pub struct ExploreTemplate {
    pub debug: bool,
    pub user: Option<User>,
}

#[derive(Template)]
#[template(path = "base/login.html")]
pub struct LoginTemplate {
    pub debug: bool,
    pub user: Option<User>,
    pub return_to: Option<String>,
    pub message: String,
}

#[derive(Template)]
#[template(path = "base/signup.html")]
pub struct SignupTemplate {
    pub debug: bool,
    pub user: Option<User>,
}
