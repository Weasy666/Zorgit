use std::path::Path;
use rocket::Route;
use rocket::http::Status;
use rocket::request::Form;
use rocket::response::Redirect;
use crate::DbConn;
use crate::models::{Config, User, DotFile, Response, Sha1, Project, SourceEntry, Topics};
use crate::models::template::{Tab, CodeTemplate, CommitTemplate, CommitsTemplate};
use crate::vcs::{Commit, SourceEntries};

//##### Routes #####//
// ✔ [get]  /{user|org}/{project}
// ✔ [get]  /{user|org}/{project}/commits/{branch}
// ✔ [get]  /{user|org}/{project}/commits/{sha}
// 🛠 [post]  /{user|org}/{project}/topics/new

pub fn get_routes() -> Vec<Route> {
    routes![
        code_get,
        commits_branch_get,
        commit_get,
        topics_new_put,
    ]
}

#[get("/<owner>/<projectname>", rank = 12)]
pub fn code_get(logged_user: Option<User>, conn: DbConn, owner: String, projectname: DotFile) -> Response<CodeTemplate> {
    // If "<projectname>.git" was requested from the browser, we strip the extension an redirect to the correct url
    if projectname.extension().is_some() && projectname.extension().unwrap() == std::ffi::OsStr::new("git") {
        let project_name = projectname.file_stem().unwrap();;
        return Response::Redirect(Redirect::to(uri!(code_get: owner = owner, projectname = project_name)));
    }
    
    let owner = User::by_name(&conn, &owner)?;
    let mut project = owner.get_or_find_project(&conn, projectname.to_str().unwrap())?;
    let collaborators = project.all_collaborators(&conn)?.expect("No collaborators found");

    if project.is_private &&
       (logged_user.is_none() || logged_user.as_ref().unwrap() != &owner || !collaborators.contains(&logged_user.as_ref().unwrap())) {
        return Response::Redirect(Redirect::to(uri!(crate::routes::login_get: return_to=format!("/{}/{}", owner.username, project.name))));
    }

    let mut branch = None;
    let mut branches = None;
    let mut num_commits = 0;
    let num_contributors = 1;//project.contributors_count(&branch).unwrap();
    let mut entries = None;
    let mut last_commit = None;
    let mut license = None;
    let mut readme = None;
    if !project.is_empty {
        branch = project.default_branch(&conn)?;
        branches = project.branches()?;
        num_commits = project.branch_commits_count(branch.as_ref().unwrap())?;
        entries = project.branch_entries(&conn, &branch.as_ref().unwrap())?;
        license = project.branch_entry_by_path(&conn, &branch.as_ref().unwrap(), Path::new("LICENSE"))
                    .ok().flatten()
                    .map(|mut v| v.pop().unwrap().1);
        if let Some(e) = &entries {
            let stuff = load_from_entries(&conn, &project, "", e, &branch.as_ref().unwrap());
            last_commit = stuff.0;
            readme = stuff.1;
        }
    }

    Response::Ok(CodeTemplate {
        debug: Config::global().debug,
        user: logged_user,
        project,
        active_tab: Tab::Code,
        branch,
        branches,
        num_commits,
        num_contributors,
        license,
        entries,
        last_commit,
        readme,
        is_file: false // we are at the root of the source code and want to see the folder structure in any case
    })
}

#[get("/<owner>/<projectname>/commits/<branch>", rank=9)]
pub fn commits_branch_get(logged_user: Option<User>, conn: DbConn, owner: String, projectname: String, branch: String) -> Response<CommitsTemplate> {
    let owner = User::by_name(&conn, &owner)?;
    let project = owner.get_or_find_project(&conn, &projectname)?;
    let collaborators = project.all_collaborators(&conn)?.expect("No collaborators found");

    if project.is_empty {
        return Response::Status(Status::NotFound);
    }

    if project.is_private &&
       (logged_user.is_none() || logged_user.as_ref().unwrap() != &owner || !collaborators.contains(&logged_user.as_ref().unwrap())) {
        return Response::Redirect(Redirect::to(uri!(crate::routes::login_get: return_to=format!("/{}/{}/commits/{}", owner.username, project.name, branch))));
    }

    let branches = project.branches()?;
    let num_commits = project.branch_commits_count(&branch)?;
    let commits = project.branch_history(&conn, &branch, true)?;
    let brnch;
    if branches.is_some() && branches.as_ref().unwrap().contains(&branch) {
        brnch = Some(branch);
    }
    else {
        return Response::Status(Status::NotFound);
    }

    Response::Ok(CommitsTemplate {
        debug: Config::global().debug,
        user: logged_user,
        project,
        active_tab: Tab::None,
        branch: brnch,
        branches,
        num_commits,
        commits,
    })
}

#[get("/<owner>/<projectname>/commits/<sha>", rank=8)]
pub fn commit_get(logged_user: Option<User>, conn: DbConn, owner: String, projectname: String, sha: Sha1) -> Response<CommitTemplate> {
    let owner = User::by_name(&conn, &owner)?;
    let project = owner.get_or_find_project(&conn, &projectname)?;
    let collaborators = project.all_collaborators(&conn)?.expect("No collaborators found");

    if project.is_empty {
        return Response::Status(Status::NotFound);
    }

    if project.is_private &&
       (logged_user.is_none() || logged_user.as_ref().unwrap() != &owner || !collaborators.contains(&logged_user.as_ref().unwrap())) {
        return Response::Redirect(Redirect::to(uri!(crate::routes::login_get: return_to=format!("/{}/{}/commits/{}", owner.username, project.name, sha))));
    }

    let commit = project.commit_by_id(&conn, &sha.to_string())?;
    let branch = project.commit_associated_branches(&commit.id())?
                    .first()
                    .map(ToString::to_string);
    let diff = project.diff_to_parent(&conn, &commit)?;

    Response::Ok(CommitTemplate {
        debug: Config::global().debug,
        user: logged_user,
        project,
        //TODO: identify correct branch
        branch,
        commit,
        diff,
        active_tab: Tab::None,
    })
}

#[put("/<owner>/<projectname>/topics", data = "<topic_ids>")]
pub fn topics_new_put(logged_user: Option<User>, conn: DbConn, owner: String, projectname: String, topic_ids: Form<Topics<i32>>) -> Response<()> {
    let owner = User::by_name(&conn, &owner)?;
    let project = owner.get_or_find_project(&conn, &projectname)?;
    let collaborators = project.all_collaborators(&conn)?.expect("No collaborators found");
    
    if project.is_private &&
       (logged_user.is_none() || logged_user.as_ref().unwrap() != &owner || !collaborators.contains(&logged_user.as_ref().unwrap())) {
        return Response::Redirect(Redirect::to(uri!(crate::routes::login_get: return_to=format!("/{}/{}", owner.username, project.name))));
    }

    project.add_topics_by_id(&conn, &topic_ids.values)?;

    Response::Ok(())
}

pub(super) fn load_from_entries<P: AsRef<Path>>(conn: &DbConn, project: &Project, root: P, entries: &SourceEntries, branch: &str) -> (Option<Commit>,Option<SourceEntry>,bool) {
    let is_file = entries.len() <= 1 && root.as_ref().file_name() == Some(std::ffi::OsStr::new(&entries.get(0).unwrap().1.name));
    let mut commit: Option<Commit> = None;
    let mut readme = None;

    for (path, entry) in entries {
        if entry.name.to_lowercase() == "readme.md" {
            readme = project.branch_entry_by_path(conn, branch, &Path::new(&path))
                        .expect("Could not load branch entry")
                        .map(|mut v| v.pop().unwrap().1);
        }
        if commit.as_ref().is_some() {
            if commit.as_ref().unwrap().time() < entry.last_commit.as_ref().unwrap().time() {
                commit = entry.last_commit.clone();
            }
        }
        else {
            commit = entry.last_commit.clone();
        }
    }

    (commit, readme, is_file)
}
