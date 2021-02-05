use std::path::{Path, PathBuf};
use rocket::http::{ContentType, Status};
use rocket::Route;
use rocket::response::Redirect;
use crate::DbConn;
use crate::models::{Config, DotFile, Response, User};
use crate::models::template::{Tab, BranchesTemplate, CodeTemplate};

//##### Routes #####//
// ✔ [get]  /{user|org}/{project}/branches/{branch}
// ✔ [get]  /{user|org}/{project}/branches/{branch}/{file|folder..}
// ✔ [get]  /{user|org}/{project}/branches/{file|folder..} - mime="image/*"
// ✔ [get]  /{user|org}/{project}/branches/{branch}/{file|folder..} - mime="image/*"

pub fn get_routes() -> Vec<Route> {
    routes![
        branches_get,
        branch_get,
        branch_entry_get,
        branch_entry_get_raw,
        entry_get_raw,
    ]
}

#[get("/<owner>/<projectname>/branches")]
pub fn branches_get(logged_user: Option<User>, conn: DbConn, owner: String, projectname: String) -> Response<BranchesTemplate> {
    let owner = User::by_name(&conn, &owner)?;
    let project = owner.get_or_find_project(&conn, &projectname)?;
    let collaborators = project.all_collaborators(&conn)?.expect("No collaborators found, but should at least be the owner");

    if project.is_empty {
        return Response::Status(Status::NotFound);
    }
    
    if project.is_private &&
       (logged_user.is_none() || logged_user.as_ref().unwrap() != &owner || !collaborators.contains(&logged_user.as_ref().unwrap())) {
        return Response::Redirect(Redirect::to(uri!(crate::routes::login_get: return_to=format!("/{}/{}/branches", owner.username, project.name))));
    }

    let branches = if let Some(brs) = project.branches()? {
            let mut brnchs = Vec::with_capacity(brs.len());
            for b in brs {
                let commit = project.branch_last_commit(&conn, &b)?;
                brnchs.push((b, commit));
            }
            Some(brnchs)
        }
        else {
            None
        };

    Response::Ok(BranchesTemplate {
        debug: Config::global().debug,
        user: logged_user,
        project,
        active_tab: Tab::Code,
        branch: None,
        branches,
    })
}

#[get("/<owner>/<projectname>/branches/<branch>")]
pub fn branch_get(logged_user: Option<User>, conn: DbConn, owner: String, projectname: String, branch: String) -> Response<CodeTemplate> {
    let owner = User::by_name(&conn, &owner)?;
    let project = owner.get_or_find_project(&conn, &projectname)?;
    let collaborators = project.all_collaborators(&conn)?.expect("No collaborators found");

    if project.is_empty {
        return Response::Status(Status::NotFound);
    }

    if project.is_private &&
       (logged_user.is_none() || logged_user.as_ref().unwrap() != &owner || !collaborators.contains(&logged_user.as_ref().unwrap())) {
        return Response::Redirect(Redirect::to(uri!(crate::routes::login_get: return_to=format!("/{}/{}/branches/{}", owner.username, project.name, branch))));
    }

    let branches = project.branches()?;
    let num_commits = project.branch_commits_count(&branch)?;
    let num_contributors = 1;//project.contributors_count(&branch).unwrap();
    let entries = project.branch_entries(&conn, &branch)?;
    let license = project.branch_entry_by_path(&conn, &branch, Path::new("LICENSE"))
                    .ok().flatten()
                    .map(|mut v| v.pop().unwrap().1);
    let mut last_commit = None;
    let mut readme = None;
    let mut is_file = true;
    if let Some(e) = &entries {
        let stuff = super::code::load_from_entries(&conn, &project, "", e, &branch);
        last_commit = stuff.0;
        readme = stuff.1;
        is_file = stuff.2;
    }

    Response::Ok(CodeTemplate {
        debug: Config::global().debug,
        user: logged_user,
        project,
        active_tab: Tab::Code,
        branch: Some(branch),
        branches,
        num_commits,
        num_contributors,
        license,
        entries,
        last_commit,
        readme,
        is_file,
    })
}

#[get("/<owner>/<projectname>/branches/<branch>/<entry..>", rank = 13)]
pub fn branch_entry_get(logged_user: Option<User>, conn: DbConn, owner: String, projectname: String, branch: String, entry: DotFile) -> Response<CodeTemplate> {
    let owner = User::by_name(&conn, &owner)?;
    let project = owner.get_or_find_project(&conn, &projectname)?;
    let collaborators = project.all_collaborators(&conn)?.expect("No collaborators found");

    if project.is_empty {
        return Response::Status(Status::NotFound);
    }

    if project.is_private &&
       (logged_user.is_none() || logged_user.as_ref().unwrap() != &owner || !collaborators.contains(&logged_user.as_ref().unwrap())) {
        return Response::Redirect(Redirect::to(uri!(crate::routes::login_get: return_to=format!("/{}/{}/branches/{}/{}", owner.username, project.name, branch, entry.to_str().unwrap()))));
    }

    let branches = project.branches()?;
    let num_commits = project.branch_commits_count(&branch)?;
    let num_contributors = 1;//project.contributors_count(&branch).unwrap();
    let entries = project.branch_entry_by_path(&conn, &branch, &entry.0)?;
    let license = project.branch_entry_by_path(&conn, &branch, Path::new("LICENSE"))?
                    .map(|mut v| v.pop().unwrap().1);
    let mut last_commit = None;
    let mut readme = None;
    let mut is_file = true;
    if let Some(e) = &entries {
        let stuff = super::code::load_from_entries(&conn, &project, &entry.0, e, &branch);
        last_commit = stuff.0;
        readme = stuff.1;
        is_file = stuff.2;
    }

    Response::Ok(CodeTemplate {
        debug: Config::global().debug,
        user: logged_user,
        project,
        active_tab: Tab::Code,
        branch: Some(branch),
        branches,
        num_commits,
        num_contributors,
        license,
        entries,
        last_commit,
        readme,
        is_file,
    })
}

#[get("/<owner>/<projectname>/<entry..>", format = "image/*", rank = 14)]
pub fn entry_get_raw<'r>(logged_user: Option<User>, conn: DbConn, owner: String, projectname: String, entry: PathBuf) -> Response<rocket::Response<'r>> {
    branch_entry_get_raw(logged_user, conn, owner, projectname, "master".to_string(), entry)
}

#[get("/<owner>/<projectname>/branches/<branch>/<entry..>", format = "image/*", rank = 12)]
pub fn branch_entry_get_raw<'r>(_logged_user: Option<User>, conn: DbConn, owner: String, projectname: String, branch: String, entry: PathBuf) -> Response<rocket::Response<'r>> {
    let owner = User::by_name(&conn, &owner)?;
    let project = owner.get_or_find_project(&conn, &projectname)?;
    let data = project.raw_branch_entry_by_path(&branch, &entry)?;
    let content_type = match entry.extension() {
        Some(ext) => ContentType::from_extension(ext.to_str().unwrap()).unwrap_or(ContentType::Binary),
        None => ContentType::Binary,
    };

    match data {
        Some(data) => Response::Ok(rocket::Response::build()
            .header(content_type)
            .sized_body(std::io::Cursor::new(data))
            .finalize()),
        None => Response::Status(Status::NotFound),
    }
}
