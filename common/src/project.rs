use std::path::PathBuf;
use rocket::request::FromParam;
use time::OffsetDateTime;
use crate::{Id, Url, entities::Owner};

pub struct Project {
    pub id: Id,
    pub owner: Owner,
    pub name: String,
    pub description: Option<String>,
    pub website: Option<Url>,
    pub default_branch: Option<String>,

    pub num_watches: u32,
    pub num_stars: u32,
    pub num_forks: u32,
    pub num_issues: u32,
    pub num_issues_closed: u32,
    pub num_issues_open: u32,
    pub num_labels: u32,
    pub num_pull_reqs: u32,
    pub num_pull_reqs_closed: u32,
    pub num_pull_reqs_open: u32,
    pub num_milestones: u32,
    pub num_milestones_closed: u32,
    pub num_milestones_open: u32,
    pub num_releases: u32,

    pub is_private: bool,
    /// Indicates whether the VCS(Version Control System) is initialized. Meaning if there is already any code committed to the system.
    pub is_empty: bool,
    pub is_archived: bool,

    pub is_fork: bool,
    pub forked_project: Option<Id>,
    pub disk_size: usize,
    //pub topics: Vec<Topic>, //TODO: Add topics

    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,

    pub dir: PathBuf,
}

impl<'r> FromParam<'r> for Project {
    type Error = Box<dyn std::error::Error>;

    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        todo!()
    }
}
