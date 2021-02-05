use crate::models::{User};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Commit {
    pub id: String,
    pub tree_id: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub time: DateTime<Utc>,
    pub author: Option<User>,
    pub committer: Option<User>,
    pub parents_id: Option<Vec<String>>,
}

impl Commit {
    pub fn id(&self) -> String {
        self.id.to_string()
    }

    pub fn short_id(&self) -> String {
        self.id.get(0..=7).unwrap().to_string()
    }

    pub fn title(&self) -> Option<String> {
        self.title.as_ref().map(|s| s.to_string())
    }

    pub fn description(&self) -> Option<String> {
        self.description.as_ref().map(|s| s.to_string())
    }
    
    pub fn author(&self) -> Option<User> {
        self.author.clone()
    }
    
    pub fn committer(&self) -> Option<User> {
        self.committer.clone()
    }
    
    pub fn parents_id(&self) -> Option<Vec<String>> {
        self.parents_id.clone()
    }
    
    pub fn parents_short_id(&self) -> Option<Vec<String>> {
        self.parents_id
            .as_ref()
            .map(|v| v.into_iter().map(|id| { id.get(0..=7).unwrap().to_string() }).collect())
    }

    pub fn time(&self) -> DateTime<Utc> {
        self.time
    }
}