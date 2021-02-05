use rocket::Route;
use rocket::http::Cookies;
use rocket::request::LenientForm;
use rocket::response::Redirect;
use crate::DbConn;
use crate::db;
use crate::models::{Config, User, UpdateAccount, DeleteEmail, NewEmail, UpdateEmail, UpdatePassword, UpdateProfile, Response, Session, Sha1};
use crate::models::template::{Tab, DashboardIssuesTemplate, DashboardPullsTemplate, UserTemplate, UserSettingsProfileTemplate, UserSettingsAccountTemplate, UserSettingsPreferencesTemplate, UserSettingsProjectsTemplate};

//##### Routes #####//
// 🛠  [get]                /{user|org}
// 🛠  [get]                /{user|org}/confirm
// ✔  [get]                /{user|org}/issues
// ❌ [get]                /{user|org}/pulls
// ✔  [get|patch]          /{user|org}/settings/profile
// ✔  [get|patch]          /{user|org}/settings/account
// ✔  [put]                /{user|org}/settings/account/password
// ✔  [post|patch|delete]  /{user|org}/settings/account/email
// ❌ [get]                /{user|org}/settings/preferences
// ❌ [get]                /{user|org}/settings/projects

pub fn get_routes() -> Vec<Route> {
    routes![
        user_get,
        user_confirm_get,
        user_issues_get,
        user_pulls_get,
        user_settings_profile_get,
        user_settings_profile_patch,
        user_settings_account_get,
        user_settings_account_patch,
        user_settings_account_password_patch,
        user_settings_account_email_post,
        user_settings_account_email_patch,
        user_settings_account_email_delete,
        user_settings_preferences_get,
        user_settings_projects_get,
    ]
}

#[get("/<user>")]
pub fn user_get(logged_user: Option<User>, conn: DbConn, user: String) -> UserTemplate {
    let user = User::by_name(&conn, &user).unwrap();

    UserTemplate {
        debug: Config::global().debug,
        user: logged_user,
        profile_user: user,
    }
}

#[get("/<user>/confirm/<token>", rank = 5)]
pub fn user_confirm_get(logged_user: Option<User>, conn: DbConn, mut cookies: Cookies<'_>, user: String, token: Sha1) -> Response<()> {
    let user = User::by_name(&conn, &user).unwrap();
    
    if (logged_user.is_none() && user.email.is_activated()) || (logged_user.is_some() && logged_user.as_ref().unwrap() != &user) {
        return Response::Redirect(Redirect::to(uri!(crate::routes::login_get: return_to=format!("/{}/confirm/{}", user.username, token))));
    }

    user.activate_email_with_token(&conn, &token).expect(&format!("Unable to activate email with token: {}", token));
    Session::create(&conn, &mut cookies, false, user.id).expect(&format!("Unable to create Session for user: {}", user.username));

    Response::Redirect(Redirect::to(uri!(crate::routes::index_get)))
}

#[get("/<user>/issues")]
pub fn user_issues_get(logged_user: Option<User>, conn: DbConn, user: String) -> Response<DashboardIssuesTemplate> {
    let user = User::by_name(&conn, &user).unwrap();
    
    if logged_user.is_none() || logged_user.as_ref().unwrap() != &user {
        return Response::Redirect(Redirect::to(uri!(crate::routes::login_get: return_to=format!("/{}/issues", user.username))));
    }
    
    let (projects, issues) = db::Issue::all_for_user_with_projects(&conn, &user).unwrap();
    let num_issues_open = issues.iter().filter(|i| !i.is_closed).count() as i32;

    Response::Ok(DashboardIssuesTemplate {
        debug: Config::global().debug,
        user: logged_user,
        num_issues_open,
        num_issues_closed: (issues.len() as i32) - num_issues_open,
        projects,
        issues,
    })
}

#[get("/<user>/pulls")]
pub fn user_pulls_get(logged_user: Option<User>, conn: DbConn, user: String) -> Response<DashboardPullsTemplate> {
    let user = User::by_name(&conn, &user).unwrap();
    
    if logged_user.is_none() || logged_user.as_ref().unwrap() != &user {
        return Response::Redirect(Redirect::to(uri!(crate::routes::login_get: return_to=format!("/{}/pulls", user.username))));
    }
    
    Response::Ok(DashboardPullsTemplate {
        debug: Config::global().debug,
        user: logged_user,
    })
}

#[get("/user/settings/profile")]
pub fn user_settings_profile_get(logged_user: Option<User>, _conn: DbConn) -> Response<UserSettingsProfileTemplate> {
    if logged_user.is_none() {
        return Response::Redirect(Redirect::to(uri!(crate::routes::login_get: return_to="/user/settings/profile")));
    }

    Response::Ok(UserSettingsProfileTemplate {
        debug: Config::global().debug,
        user: logged_user,
        active_tab: Tab::Profile,
    })
}

#[patch("/user/settings/profile", data = "<update_profile>")]
pub fn user_settings_profile_patch(logged_user: Option<User>, conn: DbConn, update_profile: LenientForm<UpdateProfile>) -> Response<()> {
    if logged_user.is_none() {
        return Response::Redirect(Redirect::to(uri!(crate::routes::login_get: return_to="/user/settings/profile")));
    }

    let account_user = logged_user.unwrap();
    account_user.update_profile(&conn, &update_profile).unwrap();

    Response::Redirect(Redirect::to(uri!(crate::routes::user::user_settings_profile_get)))
}

#[get("/user/settings/account")]
pub fn user_settings_account_get(logged_user: Option<User>, conn: DbConn) -> Response<UserSettingsAccountTemplate> {
    if logged_user.is_none() {
        return Response::Redirect(Redirect::to(uri!(crate::routes::login_get: return_to="/user/settings/account")));
    }

    let account_user = logged_user.unwrap();
    let emails = account_user.all_emails(&conn).unwrap();

    Response::Ok(UserSettingsAccountTemplate {
        debug: Config::global().debug,
        user: Some(account_user),
        active_tab: Tab::Account,
        emails,
    })
}

#[patch("/user/settings/account", data = "<update_account>")]
pub fn user_settings_account_patch(logged_user: Option<User>, conn: DbConn, update_account: LenientForm<UpdateAccount>) -> Response<()> {
    if logged_user.is_none() {
        return Response::Redirect(Redirect::to(uri!(crate::routes::login_get: return_to="/user/settings/account")));
    }
    
    let account_user = logged_user.unwrap();
    account_user.update_account(&conn, &update_account).unwrap();

    Response::Redirect(Redirect::to(uri!(crate::routes::user::user_settings_account_get)))
}

#[patch("/user/settings/account/password", data = "<update_password>")]
pub fn user_settings_account_password_patch(logged_user: Option<User>, conn: DbConn, update_password: LenientForm<UpdatePassword>) -> Response<()> {
    if logged_user.is_none() {
        return Response::Redirect(Redirect::to(uri!(crate::routes::login_get: return_to="/user/settings/account")));
    }
    
    let account_user = logged_user.unwrap();
    account_user.update_password(&conn, &update_password).unwrap();

    Response::Redirect(Redirect::to(uri!(crate::routes::user::user_settings_account_get)))
}

#[post("/user/settings/account/email", data = "<new_email>")]
pub fn user_settings_account_email_post(logged_user: Option<User>, conn: DbConn, new_email: LenientForm<NewEmail>) -> Response<()> {
    if logged_user.is_none() {
        return Response::Redirect(Redirect::to(uri!(crate::routes::login_get: return_to="/user/settings/account")));
    }

    let account_user = logged_user.unwrap();
    account_user.add_email(&conn, &new_email).unwrap();

    Response::Redirect(Redirect::to(uri!(crate::routes::user::user_settings_account_get)))
}

#[patch("/user/settings/account/email", data = "<update_email>")]
pub fn user_settings_account_email_patch(logged_user: Option<User>, conn: DbConn, update_email: LenientForm<UpdateEmail>) -> Response<()> {
    if logged_user.is_none() {
        return Response::Redirect(Redirect::to(uri!(crate::routes::login_get: return_to="/user/settings/account")));
    }

    let account_user = logged_user.unwrap();
    account_user.update_email(&conn, &update_email).unwrap();

    Response::Redirect(Redirect::to(uri!(crate::routes::user::user_settings_account_get)))
}

#[delete("/user/settings/account/email", data = "<delete_email>")]
pub fn user_settings_account_email_delete(logged_user: Option<User>, conn: DbConn, delete_email: LenientForm<DeleteEmail>) -> Response<()> {
    if logged_user.is_none() {
        return Response::Redirect(Redirect::to(uri!(crate::routes::login_get: return_to="/user/settings/account")));
    }

    let account_user = logged_user.unwrap();
    account_user.delete_email(&conn, &delete_email).unwrap();

    Response::Redirect(Redirect::to(uri!(crate::routes::user::user_settings_account_get)))
}

#[get("/user/settings/preferences")]
pub fn user_settings_preferences_get(logged_user: Option<User>, _conn: DbConn) -> Response<UserSettingsPreferencesTemplate> {
    if logged_user.is_none() {
        return Response::Redirect(Redirect::to(uri!(crate::routes::login_get: return_to="/user/settings/preferences")));
    }

    Response::Ok(UserSettingsPreferencesTemplate {
        debug: Config::global().debug,
        user: logged_user,
        active_tab: Tab::Preferences,
    })
}

#[get("/user/settings/projects")]
pub fn user_settings_projects_get(logged_user: Option<User>, _conn: DbConn) -> Response<UserSettingsProjectsTemplate> {
    if logged_user.is_none() {
        return Response::Redirect(Redirect::to(uri!(crate::routes::login_get: return_to="/user/settings/projects")));
    }

    Response::Ok(UserSettingsProjectsTemplate {
        debug: Config::global().debug,
        user: logged_user,
        active_tab: Tab::Projects,
    })
}
