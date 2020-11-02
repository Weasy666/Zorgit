use std::path::{Path, PathBuf};
use crate::{Commit, Diff, Result, SourceEntry};

pub type SourceEntries = Vec<(PathBuf,SourceEntry)>;

pub enum VCS{
    Git(super::git::Repository),
}
impl VCS {
    pub fn create<P: AsRef<Path>>(vcs_type: u32, location: P, projectname: &str) -> Result<Self> {
        match vcs_type {
            0 => super::git::Repository::create(location, projectname).map(|g| VCS::Git(g)),
            n => panic!("Tried to init unknown vcs: {}", n),
        }
    }

    pub fn inner(&self) -> &impl VersionControl {
        match self {
            VCS::Git(repo) => repo,
        }
    }

    pub fn inner_mut(&mut self) -> &mut impl VersionControl {
        match self {
            VCS::Git(repo) => repo,
        }
    }
}

#[async_trait::async_trait]
pub trait VersionControl: Sized {
    type Output;
    type Server: Server;
    fn create<P: AsRef<Path>>(path: P, projectname: &str) -> Result<Self::Output>;
    fn init<P: AsRef<Path>>(&self, tmp_dir: P, username: &str, email: &str) -> Result<()>;
    fn open<P: AsRef<Path>>(path: P) -> Result<Self::Output>;
    fn last_commit_in_branch(&self, branch_name: &str) -> Result<Commit>;
    fn branch_entries(&self, branch_name: &str) -> Result<Option<SourceEntries>>;
    fn branch_entry_by_path<P: AsRef<Path>>(&self, branch_name: &str, path: P) -> Result<Option<SourceEntries>>;
    fn raw_branch_entry_by_path<P: AsRef<Path>>(&self, branch_name: &str, path: P) -> Result<Option<Vec<u8>>>;
    fn default_branch(&self) -> Result<Option<String>>;
    fn branches(&self) -> Result<Option<Vec<String>>>;
    fn commit_by_id(&self, commit_id: &str) -> Result<Commit>;
    fn branch_last_commit(&self, branch_name: &str) -> Result<Commit>;
    fn branch_history(&self, branch_name: &str, with_merge_commits: bool) -> Result<Vec<Commit>>;
    fn branch_commits_count(&self, branch_name: &str) -> Result<usize>;
    fn commit_ancestor_count(&self, commit_id: &str) -> Result<usize>;
    fn commit_associated_branches(&self, commit_id: &str) -> Result<Vec<String>>;
    fn diff_to_parent(&self, commit: &Commit) -> Result<Diff>;
    fn diff_from_to(&self, from: &Commit, to: &Commit) -> Result<Diff>;
    fn calc_size(&self) -> Result<usize>;
    fn server(&self) -> Self::Server;
}

pub trait Server {
    fn extension(&self) -> String;
    fn routes() -> Vec<rocket::Route>;
}
