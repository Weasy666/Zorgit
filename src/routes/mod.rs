use rocket::Route;
use rocket::request::{FlashMessage, Form};
use rocket::response::{Flash, Redirect};
use rocket::http::{Cookies, RawStr};
use rocket_contrib::json::Json;
use rocket_auth::{Login, Logout};
use crate::DbConn;
use crate::models::{Config, NewUser, User, Response, Session, Topic, Topics};
use crate::models::template::{DashboardTemplate, ExploreTemplate, IndexTemplate, LoginTemplate, SignupTemplate};
use crate::vcs::Server;
use std::collections::BTreeMap;

pub mod user;
pub mod project;
pub mod error_catchers;

//##### Global routes #####//
// ❓  [get]         /       (Dashboard or Index)
// ✔  [get|post]    /login
// ✔  [get|post]    /signup
// ✔  [get]         /logout
// ❌ [get]         /explore

pub fn get_routes() -> Vec<Route> {
    let mut routes = routes![
        index_get,
        login_get,
        login_post,
        signup_get,
        signup_post,
        logout_get,
        explore_get,
        topics_search_get,
        topics_new_post,
    ];
    routes.append(&mut user::get_routes());
    routes.append(&mut project::get_routes());
    routes.append(&mut crate::vcs::git::Server::routes());
    routes
}

#[get("/")]
pub fn index_get(user: Option<User>, conn: DbConn) -> Result<DashboardTemplate, IndexTemplate> {
    match user {
        Some(user) => {
            let orgs = user.all_organisations(&conn).unwrap();
            let mut projects = BTreeMap::new();
            let mut num_orgs = 0;
            for org in orgs.iter() {
                let projs = org.all_collaborations(&conn).unwrap();
                projects.insert(org.id, projs);
                if org.is_organisation { num_orgs += 1; }
            }

            Ok(DashboardTemplate {
                user: Some(user),
                debug: Config::global().debug,
                contexts: orgs,
                projects,
                num_orgs,
            })
        }
        None => Err(IndexTemplate {
                    user: None,
                    debug: Config::global().debug
                })
    }
}

#[get("/login?<return_to>")]
pub fn login_get(return_to: Option<String>, flash: Option<FlashMessage<'_, '_>>) -> LoginTemplate {
    LoginTemplate {
        user: None,
        debug: Config::global().debug,
        return_to,
        message: flash.map(|msg| msg.msg().to_string())
                    .unwrap_or_else(|| "".to_string())
    }
}

#[post("/login?<return_to>", data = "<login>")]
pub fn login_post(login: Login<User>, return_to: Option<String>) -> Result<Redirect, Flash<Redirect>> {
    let url = return_to.unwrap_or_else(|| "/".to_string());
    login.flash_redirect(url.clone(), uri!(login_get: return_to=url), "Wrong username or password".to_string())
}

#[get("/signup")]
pub fn signup_get() -> SignupTemplate {
    SignupTemplate {
        user: None,
        debug: Config::global().debug
    }
}

#[post("/signup", data = "<new_user>")]
pub fn signup_post(conn: DbConn, mut cookies: Cookies<'_>, new_user: Form<NewUser>) -> Result<Redirect, Flash<Redirect>> {
    //TODO: do some password strength checking
    let result = User::new_user(&conn, new_user.0)
        .and_then(|user| {
            // Search for maybe already existing projects
            crate::db::Project::find_and_init_for_user(&conn, &user);
            Session::create(&conn, &mut cookies, false, user.id)
        });
    
    match result {
        Ok(_session) => Ok(Redirect::to(uri!(index_get))),
        Err(_e)      => Err(Flash::error(Redirect::to(uri!(signup_get)), "Username already in use!")),
    }
}

#[get("/logout")]
pub fn logout_get(logout: Logout<User>) -> Redirect {
    logout.redirect(uri!(index_get), uri!(index_get))
}

#[get("/explore")]
pub fn explore_get(logged_user: Option<User>, _conn: DbConn) -> Response<ExploreTemplate> {

    Response::Ok(ExploreTemplate {
        user: logged_user,
        debug: Config::global().debug,
    })
}

#[get("/topics/search?<q>")]
pub fn topics_search_get(_logged_user: Option<User>, conn: DbConn, q: &RawStr) -> Response<String> {
    let results = crate::db::Topic::matches(&conn, &q).expect("Could not load topics");

    Response::Ok(results.into_iter()
        .map(|t| format!("<a name=\"t_{}\" class=\"dropdown-item\" data-value=\"{}\">{}</a>\n", t.id, t.id, t.name))
        .collect::<String>()
    )
}

#[post("/topics/new", data = "<new_topics>")]
pub fn topics_new_post(logged_user: Option<User>, conn: DbConn, new_topics: Form<Topics<String>>) -> Response<Json<Vec<Topic>>> {
    
    if logged_user.is_none() {
        return Response::Status(rocket::http::Status::Unauthorized);
    }

    let topics = crate::db::Topic::new_vec(&conn, &new_topics.values).expect("Could not insert new topics.");

    Response::Ok(Json(topics))
}