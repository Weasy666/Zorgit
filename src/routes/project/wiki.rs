use rocket::Route;
use rocket::response::Redirect;
use crate::DbConn;
use crate::models::{Config, User, Response};
use crate::models::template::{Tab, WikiTemplate};

//##### Routes #####//
// ❌ [get]  /{user|org}/{project}/wiki

pub fn get_routes() -> Vec<Route> {
    routes![
        wiki_get,
    ]
}

#[get("/<owner>/<projectname>/wiki")]
pub fn wiki_get(logged_user: Option<User>, conn: DbConn, owner: String, projectname: String) -> Response<WikiTemplate> {
    let owner = User::by_name(&conn, &owner)?;
    let project = owner.get_or_find_project(&conn, &projectname)?;
    let collaborators = project.all_collaborators(&conn)?.expect("No collaborators found");
    
    if project.is_private &&
       (logged_user.is_none() || logged_user.as_ref().unwrap() != &owner || !collaborators.contains(&logged_user.as_ref().unwrap())) {
        return Response::Redirect(Redirect::to(uri!(crate::routes::login_get: return_to=format!("/{}/{}/wiki", owner.username, project.name))));
    }

    Response::Ok(WikiTemplate {
        debug: Config::global().debug,
        user: logged_user,
        project,
        active_tab: Tab::Wiki,
        branch: None,
    })
}