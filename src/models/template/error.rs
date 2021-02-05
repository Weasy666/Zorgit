use askama::Template;
use crate::models::User;

#[derive(Template)]
#[template(path = "errors/401.html")]
pub struct Error401Template {
    pub debug: bool,
    pub user: Option<User>,
}

#[derive(Template)]
#[template(path = "errors/403.html")]
pub struct Error403Template {
    pub debug: bool,
    pub user: Option<User>,
}

#[derive(Template)]
#[template(path = "errors/404.html")]
pub struct Error404Template {
    pub debug: bool,
    pub user: Option<User>,
}

#[derive(Template)]
#[template(path = "errors/422.html")]
pub struct Error422Template {
    pub debug: bool,
    pub user: Option<User>,
}

#[derive(Template)]
#[template(path = "errors/500.html")]
pub struct Error500Template {
    pub debug: bool,
    pub user: Option<User>,
}