use crate::models::{SourceLine, SourceLineInner};
use std::ops::{Deref, DerefMut};


pub struct Diff {
    pub files_changed:  usize,
    pub insertions:     usize,
    pub deletions:      usize,
    pub files:          Vec<DiffFile>,
}

impl Diff {
    /// Adds a new file to the diff. All afterwards added hunks and lines go into this file.
    pub fn add_file(&mut self, file: DiffFile) {
        self.files.push(file);
    }

    /// Adds a hunk to the last added file
    pub fn add_hunk(&mut self, hunk: DiffHunk) {
        self.files.last_mut().unwrap().add_hunk(hunk);
    }

    /// Adds a line as addition to the last added hunk
    pub fn add_line_addition(&mut self, line: DiffLineInner) {
        self.files.last_mut().unwrap().add_addition(DiffLine::Add(line.into()));
    }

    /// Adds a line as deletion to the last added hunk
    pub fn add_line_deletion(&mut self, line: DiffLineInner) {
        self.files.last_mut().unwrap().add_deletion(DiffLine::Del(line.into()));
    }

    /// Adds a plain line to the last added hunk
    pub fn add_line_plain(&mut self, line: DiffLineInner) {
        self.files.last_mut().unwrap().add_plain(DiffLine::Plain(line.into()));
    }

    /// Adds a binary line to the last added hunk. The line is actually just a placeholder
    /// that can be usefull as an indicator.
    pub fn add_line_binary(&mut self, line: DiffLineInner) {
        self.files.last_mut().unwrap().add_binary(DiffLine::Binary(line.into()));
    }

    /// Add a diffline to the last file. Depending on the lines type, a new hunk
    /// will be created, or the line will be added to the last hunk.
    pub fn add_to_last_file(&mut self, line: DiffLine) {
        self.files.last_mut().unwrap().add_to_last_hunk(line);
    }
}

pub enum DiffFile {
	Add(DiffFileInner),
	Change(DiffFileInner),
	Del(DiffFileInner),
	Rename(DiffFileInner),
}

impl DiffFile {
    pub fn add_to_last_hunk(&mut self, line: DiffLine) {
        match line {
            DiffLine::Hunk(_)   => self.add_hunk(DiffHunk::from(line)),
            DiffLine::Add(_)    => self.add_addition(line),
	        DiffLine::Del(_)    => self.add_deletion(line),
            DiffLine::Plain(_)  => self.add_plain(line),
            DiffLine::Binary(_) => self.add_binary(line),
        };
    }

    fn add_hunk(&mut self, hunk: DiffHunk) {
        self.hunks.push(hunk);
    }

    fn add_addition(&mut self, line: DiffLine) {
        self.addition += 1;
        self.hunks.last_mut().unwrap().add_line(line);
    }

    fn add_deletion(&mut self, line: DiffLine) {
        self.deletion += 1;
        self.hunks.last_mut().unwrap().add_line(line);
    }

    fn add_plain(&mut self, line: DiffLine) {
        self.hunks.last_mut().unwrap().add_line(line);
    }

    fn add_binary(&mut self, line: DiffLine) {
        self.hunks.push(DiffHunk::from(line));
    }

    pub fn get_title(&self) -> String {
        let empty = String::default();
        let old = self.old_name.as_ref().unwrap_or(&empty);
        let new = self.name.as_ref().unwrap_or(&empty);
        match self {
            DiffFile::Add(_)    => format!("{} {}", new, "added"),
            DiffFile::Del(_)    => format!("{} {}", old, "deleted"),
            DiffFile::Change(_) => format!("{} {}", new, "changed"),
            DiffFile::Rename(_) => {
                let old_name = old.rsplitn(2, "/").next().unwrap();
                let new_name = new.rsplitn(2, "/").next().unwrap();
                let action = if old_name == new_name { "moved" } else { "renamed" };
                format!("{} {} → {}", action, old, new)
            },
        }
    }

    pub fn hunks_highlighted(&self) -> Vec<DiffHunk> {
        crate::utils::render::highlight_hunks_by_extension(&self.hunks, self.extension.as_deref())
    }

    pub fn calc_bar_width(&self) -> f32 {
        let changes = (self.addition + self.deletion) as f32;
        100. / changes * self.addition as f32
    }
}

impl Deref for DiffFile {
    type Target = DiffFileInner;

    fn deref(&self) -> &Self::Target {
        match self {
            DiffFile::Add(f)    => f,
            DiffFile::Change(f) => f,
            DiffFile::Del(f)    => f,
            DiffFile::Rename(f) => f,
        }
    }
}

impl DerefMut for DiffFile {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            DiffFile::Add(f)    => f,
            DiffFile::Change(f) => f,
            DiffFile::Del(f)    => f,
            DiffFile::Rename(f) => f,
        }
    }
}

pub struct DiffFileInner {
    pub name:           Option<String>,
	pub old_name:       Option<String>,
    pub extension:      Option<String>,
	pub addition:       usize,
    pub deletion:       usize,
	pub is_lfs_file:    bool,
	//pub is_submodule:   bool,
	pub hunks:          Vec<DiffHunk>,
	//pub is_incomplete:  bool,
}


#[derive(Debug, Clone)]
pub struct DiffHunk {
    pub header:     DiffLine,
    pub lines:      Vec<DiffLine>,
}

impl DiffHunk {
    pub fn from(line: DiffLine) -> Self {
        DiffHunk {
            header: line,
            lines:  Vec::new(),
        }
    }

    pub fn from_inner(line: DiffLineInner) -> Self {
        DiffHunk {
            header: DiffLine::Hunk(line),
            lines:  Vec::new(),
        }
    }

    pub fn add_line(&mut self, line: DiffLine) {
        self.lines.push(line);
    }
}

pub type DiffLine = SourceLine;
pub type DiffLineInner = SourceLineInner;
