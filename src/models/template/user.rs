use std::borrow::Borrow;
use askama::Template;
use crate::models::{self, Email, User, template::Tab};


#[derive(Template, Debug)]
#[template(path = "user/profile.html")]
pub struct UserTemplate {
    pub debug: bool,
    pub user: Option<User>,
    pub profile_user: User,
}

#[derive(Template)]
#[template(path = "user/settings/profile.html")]
pub struct UserSettingsProfileTemplate {
    pub debug: bool,
    pub user: Option<User>,
    pub active_tab: Tab,
}

#[derive(Template)]
#[template(path = "user/settings/account.html")]
pub struct UserSettingsAccountTemplate {
    pub debug: bool,
    pub user: Option<User>,
    pub active_tab: Tab,
    pub emails: Vec<Email>,
}

#[derive(Template)]
#[template(path = "user/settings/preferences.html")]
pub struct UserSettingsPreferencesTemplate {
    pub debug: bool,
    pub user: Option<User>,
    pub active_tab: Tab,
}

#[derive(Template)]
#[template(path = "user/settings/projects.html")]
pub struct UserSettingsProjectsTemplate {
    pub debug: bool,
    pub user: Option<User>,
    pub active_tab: Tab,
}