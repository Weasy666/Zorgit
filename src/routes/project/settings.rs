use rocket::Route;
use rocket::request::LenientForm;
use rocket::response::Redirect;
use crate::DbConn;
use crate::models::{Config, User, Response, BasicSettings, ProjectDelete, ProjectTransfer};
use crate::models::template::{Tab, SettingsProjectTemplate, SettingsCollaborationTemplate, SettingsBranchesTemplate};

//##### Routes #####//
// ✔  [get|post] /project/new
// ❌ [get]      /{user|org}/{project}/settings
// ❌ [get]      /{user|org}/{project}/settings/collaboration
// ❌ [get]      /{user|org}/{project}/settings/branches

pub fn get_routes() -> Vec<Route> {
    let routes = routes![
        settings_project_get,
        settings_project_put,
        settings_project_delete,
        settings_project_transfer_post,
        settings_collaboration_get,
        settings_branches_get,
    ];
    routes
}

#[get("/<owner>/<projectname>/settings")]
pub fn settings_project_get(logged_user: Option<User>, conn: DbConn, owner: String, projectname: String) -> Response<SettingsProjectTemplate> {
    let owner = User::by_name(&conn, &owner)?;
    let project = owner.get_or_find_project(&conn, &projectname)?;
    let collaborators = project.all_collaborators(&conn)?.expect("No collaborators found");
    
    // TODO: need a permission system for access rights
    if project.is_private &&
       (logged_user.is_none() || logged_user.as_ref().unwrap() != &owner || !collaborators.contains(&logged_user.as_ref().unwrap())) {
        return Response::Redirect(Redirect::to(uri!(crate::routes::login_get: return_to=format!("/{}/{}/settings", owner.username, projectname))));
    }

    Response::Ok(SettingsProjectTemplate {
        debug: Config::global().debug,
        user: logged_user,
        project,
        active_tab: Tab::Settings,
        branch: None,
        settings_tab: Tab::Project,
    })
}

#[put("/<owner>/<projectname>/settings", data = "<basic_settings>")]
pub fn settings_project_put(logged_user: Option<User>, conn: DbConn, owner: String, projectname: String, basic_settings: LenientForm<BasicSettings>) -> Response<()> {
    let owner = User::by_name(&conn, &owner)?;
    let project = owner.get_or_find_project(&conn, &projectname)?;
    //let collaborators = project.all_collaborators(&conn)?.expect("No collaborators found");
    
    // TODO: need a permission system for access rights
    if project.is_private &&
       (logged_user.is_none() || logged_user.as_ref().unwrap() != &owner /*|| not project-admin */) {
        return Response::Redirect(Redirect::to(uri!(crate::routes::login_get: return_to=format!("/{}/{}/settings", owner.username, projectname))));
    }

    project.update_basic_setting(&conn, &basic_settings)?;

    Response::Redirect(Redirect::to(uri!(crate::routes::project::settings::settings_project_get: owner = &owner.username, projectname = &basic_settings.project_name)))
}

#[delete("/<owner>/<projectname>/settings", data = "<project_delete>")]
pub fn settings_project_delete(logged_user: Option<User>, conn: DbConn, owner: String, projectname: String, project_delete: LenientForm<ProjectDelete>) -> Response<()> {
    let owner = User::by_name(&conn, &owner)?;
    let project = owner.get_or_find_project(&conn, &projectname)?;
    //let collaborators = project.all_collaborators(&conn)?.expect("No collaborators found");
    
    // TODO: need a permission system for access rights
    if project.is_private &&
       (logged_user.is_none() || logged_user.as_ref().unwrap() != &owner /*|| not project-admin */) {
        return Response::Redirect(Redirect::to(uri!(crate::routes::login_get: return_to=format!("/{}/{}/settings", owner.username, projectname))));
    }

    let confirm_comparer = format!("{}/{}", &project.owner.username, &project.name);
    if project.id != project_delete.id || confirm_comparer != project_delete.confirm_name {
        // TODO: flash redirect with error message
        return Response::Redirect(Redirect::to(uri!(crate::routes::login_get: return_to=format!("/{}/{}/settings", owner.username, projectname))))
    }

    project.delete(&conn)?;

    Response::Redirect(Redirect::to(uri!(crate::routes::index_get)))
}

#[post("/<owner>/<projectname>/settings/transfer", data = "<transfer>")]
pub fn settings_project_transfer_post(logged_user: Option<User>, conn: DbConn, owner: String, projectname: String, transfer: LenientForm<ProjectTransfer>) -> Response<()> {
    let owner = User::by_name(&conn, &owner)?;
    let project = owner.get_or_find_project(&conn, &projectname)?;
    //let collaborators = project.all_collaborators(&conn)?.expect("No collaborators found");
    
    // TODO: need a permission system for access rights
    if project.is_private &&
       (logged_user.is_none() || logged_user.as_ref().unwrap() != &owner /*|| not project-admin */) {
        return Response::Redirect(Redirect::to(uri!(crate::routes::login_get: return_to=format!("/{}/{}/settings", owner.username, projectname))));
    }

    if project.name != transfer.confirm_name {
        // TODO: flash redirect with error message
        return Response::Redirect(Redirect::to(uri!(crate::routes::login_get: return_to=format!("/{}/{}/settings", owner.username, projectname))))
    }

    let new_owner = crate::db::User::by_name(&conn, &transfer.new_owner).expect("No user with specified name found.");
    project.transfer_ownership(&conn, &new_owner)?;

    Response::Redirect(Redirect::to(uri!(crate::routes::index_get)))
}

#[get("/<owner>/<projectname>/settings/collaboration")]
pub fn settings_collaboration_get(logged_user: Option<User>, conn: DbConn, owner: String, projectname: String) -> Response<SettingsCollaborationTemplate> {
    let owner = User::by_name(&conn, &owner)?;
    let project = owner.get_or_find_project(&conn, &projectname)?;
    let collaborators = project.all_collaborators(&conn)?.expect("No collaborators found");
    
    // TODO: need a permission system for access rights
    if project.is_private &&
       (logged_user.is_none() || logged_user.as_ref().unwrap() != &owner || !collaborators.contains(&logged_user.as_ref().unwrap())) {
        return Response::Redirect(Redirect::to(uri!(crate::routes::login_get: return_to=format!("/{}/{}/settings", owner.username, projectname))));
    }

    Response::Ok(SettingsCollaborationTemplate {
        debug: Config::global().debug,
        user: logged_user,
        project,
        active_tab: Tab::Settings,
        branch: None,
        settings_tab: Tab::Collaboration,
    })
}

#[get("/<owner>/<projectname>/settings/branches")]
pub fn settings_branches_get(logged_user: Option<User>, conn: DbConn, owner: String, projectname: String) -> Response<SettingsBranchesTemplate> {
    let owner = User::by_name(&conn, &owner)?;
    let project = owner.get_or_find_project(&conn, &projectname)?;
    let collaborators = project.all_collaborators(&conn)?.expect("No collaborators found");
    
    // TODO: need a permission system for access rights
    if project.is_private &&
       (logged_user.is_none() || logged_user.as_ref().unwrap() != &owner || !collaborators.contains(&logged_user.as_ref().unwrap())) {
        return Response::Redirect(Redirect::to(uri!(crate::routes::login_get: return_to=format!("/{}/{}/settings", owner.username, projectname))));
    }

    Response::Ok(SettingsBranchesTemplate {
        debug: Config::global().debug,
        user: logged_user,
        project,
        active_tab: Tab::Settings,
        branch: None,
        settings_tab: Tab::Branches,
    })
}