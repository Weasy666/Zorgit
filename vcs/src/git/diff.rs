use std::str;
use std::path::PathBuf;
use git2::{DiffFindOptions, DiffFormat, DiffOptions, Oid, Repository};
use crate::{Commit, Diff, DiffFile, DiffFileInner, DiffHunk, DiffLineInner, Result};


pub fn diff_from_to(repo: &Repository, from: Option<&Commit>, to: Option<&Commit>) -> Result<Diff> {
    let mut old = None;
    let mut new = None;
    if let Some(from) = from {
        old = repo.find_commit(Oid::from_str(&from.id)?)?.into();
    }
    if let Some(to) = to {
        new = repo.find_commit(Oid::from_str(&to.id)?)?.into();
    }
    _diff_from_to(repo, old.as_ref(), new.as_ref())
}

pub fn _diff_from_to<'a>(repo: &'a Repository, from: Option<&git2::Commit<'a>>, to: Option<&git2::Commit<'a>>) -> Result<Diff> {
    let mut diff_opts = DiffOptions::new();
    let mut diff_find_opts = DiffFindOptions::new();
    diff_find_opts.renames(true);
    let mut old = None;
    let mut new = None;
    if let Some(from) = from {
        old = from.tree()?.into();
    }
    if let Some(to) = to {
        new = to.tree()?.into();
    }
    let mut diff2 = repo.diff_tree_to_tree(new.as_ref(), old.as_ref(), Some(&mut diff_opts))?;
    diff2.find_similar(Some(&mut diff_find_opts))?;

    let stats = diff2.stats()?;
    let mut diff = Diff {
        files_changed:  stats.files_changed(),
        insertions:     stats.insertions(),
        deletions:      stats.deletions(),
        //TODO: check if files_changed() can be used to create Vec with capacity
        files:          Vec::new(),
    };

    diff2.print(DiffFormat::Patch, |_delta, _hunk, line| {
        match line.origin() {
            // File header
            'F' =>  diff.add_file(line.into()),
            // Hunk header
            'H' =>  diff.add_hunk(DiffHunk::from_inner(line.into())),
            // Line context
            ' ' =>  diff.add_line_plain(line.into()),
            // Line addition
            '+' =>  diff.add_line_addition(line.into()),
            // Line deletion
            '-' =>  diff.add_line_deletion(line.into()),
            // Context (End of file)
            '=' =>  diff.add_line_plain(line.into()),
            'B' =>  diff.add_line_binary(line.into()),
            _   =>  (),
            //> - Add (End of file)
            //< - Remove (End of file)
            //B - Line binary
        }

        true
    }).unwrap();

    Ok(diff)
}

impl<'a> From<git2::DiffLine<'a>> for DiffFile {
    fn from(line: git2::DiffLine<'_>) -> Self {
        let line = str::from_utf8(line.content()).unwrap();
        let mut line_split: Vec<&str> = line.trim().split('\n').collect();
        let mut name = None;
        let mut old_name = None;
        let extension;
        if line.contains("new file mode") {
            let split: Vec<&str> = line_split.first().unwrap().split(' ').collect();
            name = split.get(3).map(|s| s.replacen("b/", "", 1).to_string());
            extension = name.as_ref().map(|n| PathBuf::from(n));
        }
        else if line.contains("deleted file mode") {
            let split: Vec<&str> = line_split.first().unwrap().split(' ').collect();
            old_name = split.get(2).map(|s| s.replacen("a/", "", 1).to_string());
            extension = old_name.as_ref().map(|n| PathBuf::from(n));
        }
        else {
            name = line_split.pop()
                .filter(|s| !s.contains("/dev/null"))
                .map(|s| s.replace("+++ b/", "").trim().to_string());
            old_name = line_split.pop()
                .filter(|s| !s.contains("/dev/null"))
                .map(|s| s.replace("--- a/", "").trim().to_string());
            extension = name.as_ref().map(|n| PathBuf::from(n));
        }
        let extension = extension.map(|p| {
                        p.extension()
                            .map(|s| s.to_str())
                            .flatten()
                            .map(ToString::to_string)
                    })
                    .flatten();
        let diff_file_inner = DiffFileInner {
            name,
            old_name,
            extension,
            addition:       usize::default(),
            deletion:       usize::default(),
            is_lfs_file:    bool::default(),
            hunks:          Vec::new(),
        };

        diff_file_inner.into()
    }
}

impl From<DiffFileInner> for DiffFile {
    fn from(file_inner: DiffFileInner) -> Self {
        if file_inner.name.is_none() {
            DiffFile::Del(file_inner)
        }
        else if file_inner.old_name.is_none() {
            DiffFile::Add(file_inner)
        }
        else if file_inner.old_name == file_inner.name {
            DiffFile::Change(file_inner)
        }
        else {
            let mut file_inner = file_inner;
            file_inner.name = file_inner.name.map(|s| s.replacen("rename to ", "", 1).to_string());
            file_inner.old_name = file_inner.old_name.map(|s| s.replacen("rename from ", "", 1).to_string());
            DiffFile::Rename(file_inner)
        }
    }
}


impl<'a> From<git2::DiffLine<'a>> for DiffLineInner {
    fn from(line: git2::DiffLine<'_>) -> Self {
        DiffLineInner {
            old_num:    line.old_lineno(),
	        new_num:    line.new_lineno(),
            marker:     line.origin(),
	        content:    str::from_utf8(line.content()).ok().map(ToString::to_string),
        }
    }
}
