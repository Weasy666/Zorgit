use std::convert::TryFrom;
use anyhow::anyhow;
use time::OffsetDateTime;
use crate::{Commit, Signature};


impl<'a> TryFrom<git2::Commit<'a>> for Commit {
    type Error = Box<dyn std::error::Error>;

    fn try_from(origin: git2::Commit<'a>) -> Result<Self,Self::Error> {

        let parents = origin.parent_ids()
            .map(|id| id.to_string())
            .collect::<Vec<_>>();
        let author = Signature {
            name: origin.author().name().ok_or(anyhow!(""))?.to_string(),
            email: origin.author().email().ok_or(anyhow!(""))?.to_string()
        };
        let committer = Signature {
            name: origin.committer().name().ok_or(anyhow!(""))?.to_string(),
            email: origin.committer().email().ok_or(anyhow!(""))?.to_string()
        };

        Ok(Commit {
            id: origin.id().to_string(),
            tree_id: origin.tree_id().to_string(),
            title: origin.summary().map(ToString::to_string),
            description: origin.message()
                .and_then(|s| s.splitn(2, "\n\n").last())
                .filter(|s| !s.is_empty())
                .map(ToString::to_string),
            time: OffsetDateTime::from_unix_timestamp(origin.time().seconds()),
            author: Some(author),
            committer: Some(committer),
            parents_id: if !parents.is_empty() { Some(parents) } else { None },
        })
    }
}
