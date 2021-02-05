use rocket::Route;
use rocket::response::Redirect;
use crate::DbConn;
use crate::models::{Config, User, Response};
use crate::models::template::{Tab, PullTemplate, PullsTemplate};

//##### Routes #####//
// ❌ [get]  /{user|org}/{project}/pulls
// ❌ [get]  /{user|org}/{project}/pulls/{number}

pub fn get_routes() -> Vec<Route> {
    routes![
        pulls_get,
        pull_get,
    ]
}

#[get("/<owner>/<projectname>/pulls")]
pub fn pulls_get(logged_user: Option<User>, conn: DbConn, owner: String, projectname: String) -> Response<PullsTemplate> {
    let owner = User::by_name(&conn, &owner)?;
    let project = owner.get_or_find_project(&conn, &projectname)?;
    let collaborators = project.all_collaborators(&conn)?.expect("No collaborators found");
    
    if project.is_private &&
       (logged_user.is_none() || logged_user.as_ref().unwrap() != &owner || !collaborators.contains(&logged_user.as_ref().unwrap())) {
        return Response::Redirect(Redirect::to(uri!(crate::routes::login_get: return_to=format!("/{}/{}/pulls", owner.username, project.name))));
    }

    Response::Ok(PullsTemplate {
        debug: Config::global().debug,
        user: logged_user,
        project,
        active_tab: Tab::Pulls,
        branch: None,
        pulls: Vec::new(),
    })
}

#[get("/<owner>/<projectname>/pulls/<_pull>")]
pub fn pull_get(logged_user: Option<User>, conn: DbConn, owner: String, projectname: String, _pull: u32) -> Response<PullTemplate> {
    let owner = User::by_name(&conn, &owner)?;
    let project = owner.get_or_find_project(&conn, &projectname)?;
    let collaborators = project.all_collaborators(&conn)?.expect("No collaborators found");
    
    if project.is_private &&
       (logged_user.is_none() || logged_user.as_ref().unwrap() != &owner || !collaborators.contains(&logged_user.as_ref().unwrap())) {
        return Response::Redirect(Redirect::to(uri!(crate::routes::login_get: return_to=format!("/{}/{}/pulls/{}", owner.username, project.name, _pull))));
    }

    Response::Ok(PullTemplate {
        debug: Config::global().debug,
        user: logged_user,
        project,
        active_tab: Tab::Pulls,
        branch: None,
    })
}