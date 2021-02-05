pub mod git;
mod commit;
mod diff;
mod vcs;

pub use self::commit::Commit;
pub use self::diff::{Diff, DiffFileInner, DiffFile, DiffHunk, DiffLine, DiffLineInner};
pub use self::vcs::{SourceEntries, VCS, VersionControl, Server};