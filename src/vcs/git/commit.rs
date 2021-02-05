use crate::models::{User};
use crate::DbConn;
use crate::vcs;
use chrono::{TimeZone, Utc};


impl<'a> From<(&DbConn, git2::Commit<'a>)> for vcs::Commit {
    fn from(origin: (&DbConn, git2::Commit<'a>)) -> Self {

        let parents = origin.1.parent_ids().map(|id| id.to_string()).collect::<Vec<_>>();
        let author = origin.1.author().email().or(origin.1.author().name()).map(ToString::to_string);
        let committer = origin.1.committer().email().or(origin.1.committer().name()).map(ToString::to_string);

        vcs::Commit {
            id: origin.1.id().to_string(),
            tree_id: origin.1.tree_id().to_string(),
            title: origin.1.summary().map(ToString::to_string),
            description: origin.1.message()
                .and_then(|s| s.splitn(2, "\n\n").last())
                .filter(|s| !s.is_empty())
                .map(ToString::to_string),
            time: Utc.timestamp(origin.1.time().seconds(), 0),
            author: author.map(|a| User::by_name_or_email(origin.0, &a).ok().or(Some(User::unknown(&a)))).flatten(),
            committer: committer.map(|c| User::by_name_or_email(origin.0, &c).ok().or(Some(User::unknown(&c)))).flatten(),
            parents_id: if !parents.is_empty() { Some(parents) } else { None },
        }
    }
}