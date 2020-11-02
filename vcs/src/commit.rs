use time::OffsetDateTime;

#[derive(Debug, Clone)]
pub struct Signature {
    name: String,
    email: String,
}

#[derive(Debug, Clone)]
pub struct Commit {
    pub id: String,
    pub tree_id: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub time: OffsetDateTime,
    pub author: Option<Signature>,
    pub committer: Option<Signature>,
    pub parents_id: Option<Vec<String>>,
}
