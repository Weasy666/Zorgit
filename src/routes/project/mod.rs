use rocket::{Route, http::Status};
use rocket::request::LenientForm;
use rocket::response::Redirect;
use crate::DbConn;
use crate::models::{Config, NewProjectForm, User, Response};
use crate::models::template::{Tab, ForksTemplate, NewProjectTemplate, StarsTemplate, WatchersTemplate};

pub mod branches;
pub mod code;
pub mod issues;
pub mod pull_requests;
pub mod releases;
pub mod settings;
pub mod wiki;

//##### Routes #####//
// ✔  [get|post] /project/new
// ❌ [get]      /{user|org}/{project}/watchers
// ❌ [get]      /{user|org}/{project}/stars
// ❌ [post]     /{user|org}/{project}/star
// ❌ [post]     /{user|org}/{project}/unstar
// ❌ [get]      /{user|org}/{project}/forks
// ✔  [post]     /{user|org}/{project}/markdown

pub fn get_routes() -> Vec<Route> {
    let mut routes = routes![
        project_new_get,
        project_new_post,
        watchers_get,
        stars_get,
        star_post,
        unstar_post,
        forks_get,
        markdown_post,
    ];
    routes.append(&mut branches::get_routes());
    routes.append(&mut code::get_routes());
    routes.append(&mut issues::get_routes());
    routes.append(&mut pull_requests::get_routes());
    routes.append(&mut releases::get_routes());
    routes.append(&mut settings::get_routes());
    routes.append(&mut wiki::get_routes());
    routes
}

#[get("/project/new")]
pub fn project_new_get(logged_user: Option<User>, conn: DbConn) -> Response<NewProjectTemplate> {
    if logged_user.is_none() {
        return Response::Redirect(Redirect::to(uri!(crate::routes::login_get: return_to="/project/new")));
    }

    let owners = logged_user.as_ref().unwrap()
                    .all_organisations(&conn)?;

    Response::Ok(NewProjectTemplate {
        user: logged_user,
        debug: Config::global().debug,
        owners,
    })
}

#[post("/project/new", data = "<new_project>")]
pub fn project_new_post(logged_user: Option<User>, conn: DbConn, new_project: LenientForm<NewProjectForm>) -> Response<()> {
    if logged_user.is_none() {
        return Response::Redirect(Redirect::to(uri!(crate::routes::login_get: return_to="/project/new")));
    }
    let project = new_project.create(&conn)?;

    Response::Redirect(Redirect::to(uri!(code::code_get: owner = &logged_user.unwrap().username, projectname = project.name)))
}

#[get("/<owner>/<projectname>/watchers")]
pub fn watchers_get(logged_user: Option<User>, conn: DbConn, owner: String, projectname: String) -> Response<WatchersTemplate> {
    let owner = User::by_name(&conn, &owner)?;
    let project = owner.get_or_find_project(&conn, &projectname)?;
    let collaborators = project.all_collaborators(&conn)?.expect("No collaborators found");
    
    if project.is_private &&
       (logged_user.is_none() || logged_user.as_ref().unwrap() != &owner || !collaborators.contains(&logged_user.as_ref().unwrap())) {
        return Response::Redirect(Redirect::to(uri!(crate::routes::login_get: return_to=format!("/{}/{}/watchers", owner.username, projectname))));
    }

    Response::Ok(WatchersTemplate {
        debug: Config::global().debug,
        user: logged_user,
        project,
        active_tab: Tab::None,
        branch: None,
    })
}

#[get("/<owner>/<projectname>/stars")]
pub fn stars_get(logged_user: Option<User>, conn: DbConn, owner: String, projectname: String) -> Response<StarsTemplate> {
    let owner = User::by_name(&conn, &owner)?;
    let project = owner.get_or_find_project(&conn, &projectname)?;
    let collaborators = project.all_collaborators(&conn)?.expect("No collaborators found");
    
    if project.is_private &&
       (logged_user.is_none() || logged_user.as_ref().unwrap() != &owner || !collaborators.contains(&logged_user.as_ref().unwrap())) {
        return Response::Redirect(Redirect::to(uri!(crate::routes::login_get: return_to=format!("/{}/{}/stars", owner.username, projectname))));
    }

    Response::Ok(StarsTemplate {
        debug: Config::global().debug,
        user: logged_user,
        project,
        active_tab: Tab::None,
        branch: None,
    })
}

#[post("/<owner>/<projectname>/star")]
pub fn star_post(logged_user: Option<User>, conn: DbConn, owner: String, projectname: String) -> Response<StarsTemplate> {
    let owner = User::by_name(&conn, &owner)?;
    let project = owner.get_or_find_project(&conn, &projectname)?;
    let collaborators = project.all_collaborators(&conn)?.expect("No collaborators found");
    
    if project.is_private &&
       (logged_user.is_none() || logged_user.as_ref().unwrap() != &owner || !collaborators.contains(&logged_user.as_ref().unwrap())) {
        return Response::Redirect(Redirect::to(uri!(crate::routes::login_get: return_to=format!("/{}/{}/stars", owner.username, projectname))));
    }

    Response::Redirect(Redirect::to(uri!(crate::routes::project::code::code_get: owner=&owner.username, projectname=&project.name)))
}

#[post("/<owner>/<projectname>/unstar")]
pub fn unstar_post(logged_user: Option<User>, conn: DbConn, owner: String, projectname: String) -> Response<StarsTemplate> {
    let owner = User::by_name(&conn, &owner)?;
    let project = owner.get_or_find_project(&conn, &projectname)?;
    let collaborators = project.all_collaborators(&conn)?.expect("No collaborators found");
    
    if project.is_private &&
       (logged_user.is_none() || logged_user.as_ref().unwrap() != &owner || !collaborators.contains(&logged_user.as_ref().unwrap())) {
        return Response::Redirect(Redirect::to(uri!(crate::routes::login_get: return_to=format!("/{}/{}/stars", owner.username, projectname))));
    }

    Response::Redirect(Redirect::to(uri!(crate::routes::project::code::code_get: owner=&owner.username, projectname=&project.name)))
}

#[get("/<owner>/<projectname>/forks")]
pub fn forks_get(logged_user: Option<User>, conn: DbConn, owner: String, projectname: String) -> Response<ForksTemplate> {
    let owner = User::by_name(&conn, &owner)?;
    let project = owner.get_or_find_project(&conn, &projectname)?;
    let collaborators = project.all_collaborators(&conn)?.expect("No collaborators found");
    
    if project.is_private &&
       (logged_user.is_none() || logged_user.as_ref().unwrap() != &owner || !collaborators.contains(&logged_user.as_ref().unwrap())) {
        return Response::Redirect(Redirect::to(uri!(crate::routes::login_get: return_to=format!("/{}/{}/forks", owner.username, projectname))));
    }

    Response::Ok(ForksTemplate {
        debug: Config::global().debug,
        user: logged_user,
        project,
        active_tab: Tab::None,
        branch: None,
    })
}

#[post("/<owner>/<projectname>/markdown", data = "<data>")]
pub fn markdown_post(logged_user: Option<User>, conn: DbConn, owner: String, projectname: String, data: rocket::data::Data) -> Response<String> {
    let owner = User::by_name(&conn, &owner)?;
    let project = owner.get_or_find_project(&conn, &projectname)?;
    let collaborators = project.all_collaborators(&conn)?.expect("No collaborators found");
    
    if project.is_private &&
       (logged_user.is_none() || logged_user.as_ref().unwrap() != &owner || !collaborators.contains(&logged_user.as_ref().unwrap())) {
        return Response::Status(Status::Unauthorized);
    }
    
    let mut markdown = Vec::new();
    data.stream_to(&mut markdown).expect("Failed to write to stdin");
    Response::Ok(crate::utils::render::markdown(&projectname, std::str::from_utf8(&markdown).expect("Found invalid UTF-8")))
}