use rocket::http::Cookie;
use crate::DbConn;
use crate::models::{Config, Language, UpdateAccount, Email, DeleteEmail, Mailer, NewEmail, UpdateEmail, UpdatePassword, UpdateProfile, Project, Session, NewUser, Url, Password, Sha1};
use crate::db;
use rocket_auth::{Authenticator, Login, Logout};
use rocket::request::{FormItems, FromRequest, Request};
use rocket::outcome::Outcome;
use chrono::NaiveDateTime;
use std::path::PathBuf;
use std::ops::{Deref, DerefMut};
use anyhow::{self, Context, Result};
use data_encoding::BASE64;


#[derive(Debug, Clone)]
pub enum User {
    Individual(UserInner),
    Organisation(UserInner),
    Team(UserInner),
}

impl From<(db::User, Email)> for User {
    fn from(origin: (db::User, Email)) -> User {
        let user = UserInner {
            id: origin.0.id,
            username: origin.0.username,
            full_name: origin.0.full_name,
            avatar: origin.0.avatar,
            avatar_email: origin.0.avatar_email,
            email: origin.1,
            location: origin.0.location,
            website: origin.0.website,
            description: origin.0.description,
            language: origin.0.language,
            must_change_password: origin.0.must_change_password,
            is_email_hidden: origin.0.is_email_hidden,
            is_admin: origin.0.is_admin,
            is_organisation: origin.0.is_organisation,
            created_at: origin.0.created_at,
            updated_at: origin.0.updated_at,
            last_seen_at: origin.0.last_seen_at,
        };
        match origin.0.types {
            0 => User::Individual(user),
            1 => User::Organisation(user),
            2 => User::Team(user),
            _ => panic!("'to_model' tried to convert to unknown user type"),
        }
    }
}

impl Deref for User {
    type Target = UserInner;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Individual(user) => user,
            Self::Organisation(user) => user,
            Self::Team(user) => user,
        }
    }
}

impl DerefMut for User {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::Individual(user) => user,
            Self::Organisation(user) => user,
            Self::Team(user) => user,
        }
    }
}

#[derive(Debug, Clone)]
pub struct UserInner {
    pub id: i32,
    pub username: String,
    pub full_name: Option<String>,
    pub avatar: PathBuf,
    pub avatar_email: Option<Email>,
    /// This is the primary email. If you need all registered emails, use the function.
    pub email: Email,
    pub location: Option<String>,
    pub website: Option<Url>,
    pub description: Option<String>,
    pub language: Language,

    pub must_change_password: bool,
    pub is_email_hidden: bool,
    pub is_admin: bool,
    pub is_organisation: bool,
    
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub last_seen_at: NaiveDateTime,
}

impl User {
    pub fn unknown(email: &str) -> User {
        User::Individual(UserInner {
            id: -1,
            username: email.to_string(),
            full_name: None,
            avatar: crate::utils::create_default_avatar(email).unwrap(),
            avatar_email: None,
            email: email.parse::<Email>().expect(&format!("Not a valid email address: {}", email)),
            location: None,
            website: None,
            description: None,
            language: "en-EN".parse::<Language>().unwrap(),

            must_change_password: false,
            is_email_hidden: false,
            is_admin: false,
            is_organisation: false,
            
            created_at: NaiveDateTime::from_timestamp(0, 0),
            updated_at: NaiveDateTime::from_timestamp(0, 0),
            last_seen_at: NaiveDateTime::from_timestamp(0, 0),
        })
    }

    pub fn new_user(conn: &DbConn, new_user: NewUser) -> Result<User> {
        let user = db::User::new_user(conn, new_user)?;
        if Config::global().mailer().is_enabled() {
            Mailer::send_confirm_user(&user, user.email.token.as_ref().unwrap())?;
        }
        else {
            user.activate_email(conn, &user.email)?;
        }
        Ok(user)
    }

    pub fn add_email(&self, conn: &DbConn, new_email: &NewEmail) -> Result<Email> {
        let email = db::User::new_email(conn, self, new_email)?;
        if Config::global().mailer().is_enabled() {
            Mailer::send_confirm_additional_email(self, &email, email.token.as_ref().unwrap())?;
        }
        else {
            self.activate_email(conn, &email)?;
        }
        Ok(email)
    }

    pub fn activate_email_with_token(&self, conn: &DbConn, token: &Sha1) -> Result<()> {
        let result = db::Email::activate_for_user_with_token(conn, self, token)?;
        Ok(result)
    }

    pub fn activate_email(&self, conn: &DbConn, email: &Email) -> Result<()> {
        let result = db::Email::activate_for_user(conn, self, email)?;
        Ok(result)
    }

    pub fn update_email(&self, conn: &DbConn, update_email: &UpdateEmail) -> Result<()> {
        let result = db::User::update_email(conn, self, update_email)?;
        Ok(result)
    }

    pub fn delete_email(&self, conn: &DbConn, delete_email: &DeleteEmail) -> Result<()> {
        let result = db::User::delete_email(conn, self, delete_email)?;
        Ok(result)
    }

    pub fn all_emails(&self, conn: &DbConn) -> Result<Vec<Email>> {
        let email = db::User::all_emails(conn, self)?;
        Ok(email)
    }

    pub fn update_profile(&self, conn: &DbConn, update_profile: &UpdateProfile) -> Result<()> {
        let result = db::User::update_profile(conn, self, update_profile)?;
        Ok(result)
    }

    pub fn update_account(&self, conn: &DbConn, update_account: &UpdateAccount) -> Result<()> {
        let auth_user = db::AuthUser::by_name_or_email(&conn, &self.email.address)
            .context(format!("User with email not found: {}", self.email.address))?;
        auth_user.authenticate(&update_account.password)
            .map_err(|user| anyhow!("User with email not found: {}", user.email.address))?;
        let result = db::User::update_account(conn, self, update_account)?;
        Ok(result)
    }

    pub fn update_password(&self, conn: &DbConn, update_password: &UpdatePassword) -> Result<()> {
        let auth_user = db::AuthUser::by_name_or_email(&conn, &self.email.address)
            .context(format!("User with email not found: {}", self.email.address))?;
        auth_user.authenticate(&update_password.old)
            .map_err(|user| anyhow!("User with email not found: {}", user.email.address))?;
        let result = db::User::update_password(conn, self, update_password)?;
        Ok(result)
    }

    pub fn by_name(conn: &DbConn, username: &str) -> Result<User> {
        let user = db::User::by_name(conn, username)?;
        Ok(user)
    }

    pub fn by_name_or_email(conn: &DbConn, keyword: &str) -> Result<User> {
        let user = db::User::by_name_or_email(conn, keyword)?;
        Ok(user)
    }

    pub fn by_id(conn: &DbConn, user_id: i32) -> Result<User> {
        let user = db::User::by_id(conn, user_id)?;
        Ok(user)
    }

    pub fn project_with_name(&self, conn: &DbConn, projectname: &str) -> Result<Project> {
        let project = db::Project::by_name_for_user(conn, projectname, &self.username)?;
        Ok(project)
    }

    pub fn all_projects(&self, conn: &DbConn) -> Result<Vec<Project>> {
        let projects = db::Project::all_for_username(conn, &self.username)?;
        Ok(projects)
    }

    pub fn all_for_issue_id(conn: &DbConn, issue_id: i32) -> Result<Vec<User>> {        
        let users = db::User::all_for_issue_id(conn, issue_id)?;
        Ok(users)
    }

    pub fn update_assignees_for_issue_id(conn: &DbConn, issue_id: i32, assignees: &[User]) -> Result<Vec<User>> {
        let users = db::User::update_assignees_for_issue_id(conn, issue_id, assignees)?;
        Ok(users)
    }

    pub fn delete_assignee_for_issue_id(conn: &DbConn, issue_id: i32, assignee: &User) -> Result<()> {
        let result = db::User::delete_assignee_for_issue_id(conn, issue_id, assignee)?;
        Ok(result)
    }

    pub fn all_for_project(conn: &DbConn, project: &Project) -> Result<Vec<User>> {        
        User::all_for_project_id(conn, project.id)
    }

    pub fn all_for_project_id(conn: &DbConn, project_id: i32) -> Result<Vec<User>> {        
        let users = db::User::all_for_project_id(conn, project_id)?;
        Ok(users)
    }

    pub fn all_organisations(&self, conn: &DbConn) -> Result<Vec<User>> {
        let users = db::User::all_organisations(conn, self)?;
        Ok(users)
    }

    pub fn all_collaborations(&self, conn: &DbConn) -> Result<Vec<Project>> {
        let projects = db::User::all_collaborations(conn, self)?;
        Ok(projects)
    }

    pub fn insert_all_assignees_for_issue_id(conn: &DbConn, issue_id: i32, assignees: &[User]) -> Result<Vec<User>> {
        let users = db::User::insert_all_assignees_for_issue_id(conn, issue_id, assignees)?;
        Ok(users)
    }

    pub fn insert_assignee_by_id_for_issue_id(conn: &DbConn, issue_id: i32, assignee_id: i32)  -> Result<User> {
        let user = db::User::insert_assignee_by_id_for_issue_id(conn, issue_id, assignee_id)?;
        Ok(user)
    }

    pub fn insert_assignee_for_issue(&self, conn: &DbConn, issue_id: i32)  -> Result<User> {
        let user = db::User::insert_assignee_by_id_for_issue_id(conn, issue_id, self.id)?;
        Ok(user)
    }

    pub fn get_or_find_project(&self, conn:&DbConn, projectname: &str) -> Result<Project> {
        let project = db::User::get_or_find_project(conn, self, projectname)?;
        Ok(project)
    }

    pub fn projects(&self, conn: &DbConn) -> Result<Vec<Project>> {
        let projects = db::User::projects_for_user(conn, &self.username)?;
        Ok(projects)
    }

    pub fn project_by_name(&self, conn: &DbConn, projectname: &str) -> Result<Project> {
        let project = db::User::project_with_name_for_user(conn, projectname, &self.username)?;
        Ok(project)
    }

//##### General functions #####
    pub fn get_projects_dir(&self) -> PathBuf {
        User::projects_dir(&self.username)
    }

    pub fn projects_dir(username: &str) -> PathBuf {
        Config::global().projects_dir().join(username)
    }

    pub fn url_relative(&self) -> String {
        let url = self.url_absolute();
        if url.cannot_be_a_base() {
            url.to_string()
        }
        else {
            url.path().to_owned()
        }
    }

    pub fn url_absolute(&self) -> Url {
        if self.id == -1 {
            let mailto = format!("mailto:{}", &self.email.address);
            url::Url::parse(&mailto).unwrap().into()
        }
        else {
            Config::global().root_url().join(&self.username).unwrap().into()
        }
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Authenticator for User {
    type Error = anyhow::Error;

    fn get_cookie_key() -> String {
        Config::global().session_key()
    }

    fn authenticate(request: &Request<'_>, items: &mut FormItems<'_>, _strict: bool) -> Result<Login<Self>, Self::Error> {
        // Get the values we need from the previously extracted FormItems
        let mut username = String::default();
        let mut password = Err(anyhow!("Invalid login form with missing field 'username' or 'password'."));
        let mut remember = false;
        for form_item in items {
            let (key, value) = form_item.key_value_decoded();
            match key.as_str() {
                "username" | "email" => username = value,
                "password" if !value.is_empty() => password = value.parse::<Password>(),
                "remember" => remember = value == "on",
                _ => (),
            }
        }

        // Check that we got some usable values
        if username.is_empty() || password.is_err() {
            return Err(anyhow!("Invalid login form with missing field 'username' or 'password'."));
        }

        // Retrieve DB connection from request and authentication password
        let conn = request.guard::<DbConn>().unwrap();
        let auth_user = db::AuthUser::by_name_or_email(&conn, &username)?;
        let authenticated = auth_user.authenticate(&password?);
        
        match authenticated {
            Ok(user) => {
                Session::create(&conn, &mut request.cookies(), remember, user.id)?;
                Ok(Login::Success(user))
            },
            Err(user) => Ok(Login::Failure(user)),
        }
    }

    fn logout(request: &Request<'_>) -> Result<Logout<Self>, Self::Error> {
        // Retrieve DB connection from request
        let conn = request.guard::<DbConn>().unwrap();
        let mut cookies = request.cookies();
        let token = cookies.get_private(&Self::get_cookie_key())
            .ok_or_else(|| anyhow!("No user or session for the provided cookie found!"))?;

        cookies.remove_private(Cookie::named(Self::get_cookie_key()));

        let logout = Session::by_token(&conn, token.value())
            .and_then(|s| s.delete(&conn).map(|_| s.user));
        
        match logout {
            Ok(user) => Ok(Logout::Success(user)),
            _        => Ok(Logout::Failure),
        }
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = anyhow::Error;

    fn from_request(request: &'a Request<'r>) -> rocket::request::Outcome<User, Self::Error> {
        // Retrieve DB connection from request
        let conn = request.guard::<DbConn>().unwrap();
        let user;
        if let Some(auth_header) = request.headers().get_one("Authorization") {
            let encoded = auth_header.split(" ").nth(1).unwrap();
            let decoded = BASE64.decode(encoded.as_bytes()).unwrap();
            let user_pass = String::from_utf8(decoded).map(|s| s).unwrap();
            let mut user_pass = user_pass.splitn(2, ":");
            let auth_user = db::AuthUser::by_name_or_email(&conn, user_pass.next().unwrap()).unwrap();
            user = user_pass.next().unwrap().parse::<Password>()
                .map_err(|_| "User not found!".to_string())
                .and_then(|p| auth_user.authenticate(&p).map_err(|_| "User not found!".to_string()));
        }
        else {
            // check if sessionID is valid and get user data from DB
            user = request.cookies()
                .get_private(&Self::get_cookie_key())
                .ok_or("No Session found!".to_string())
                .and_then(|sid| Session::validate(&conn, sid.value()).map_err(|_| "No valid session found!".to_string()))
                .and_then(|session| 
                    User::by_id(&conn, session.user.id)
                    .map_err(|_| "User not found!".to_string())
                );
        }

        match user {
            Ok(user) => Outcome::Success(user),
            _        => Outcome::Forward(()),
        }
    }
}
