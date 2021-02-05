pub mod error;
mod project;
mod template;
mod user;


pub use project::*;
pub use template::*;
pub use user::*;


#[derive(PartialEq, Eq)]
pub enum Tab {
    None,
    Code,
    Issues,
    Pulls,
    Releases,
    Wiki,
    Settings,
    Profile,
    Account,
    Preferences,
    Projects,
    Project,
    Collaboration,
    Branches
}