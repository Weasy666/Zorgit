pub mod git;
mod commit;
mod diff;
mod source_entry;
mod source_line;
mod vcs;

pub use self::commit::{Commit, Signature};
pub use self::diff::{Diff, DiffFileInner, DiffFile, DiffHunk, DiffLine, DiffLineInner};
pub use self::source_entry::SourceEntry;
pub use self::source_line::{SourceLine, SourceLineInner};
pub use self::vcs::{SourceEntries, VCS, VersionControl, Server};

use std::error::Error;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;
