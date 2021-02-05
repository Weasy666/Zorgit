use rocket::Route;
use rocket::request::{Form, LenientForm};
use rocket::response::Redirect;
use crate::DbConn;
use crate::models::{Config, NewComment, NewIssue, UpdateIssue, NewLabel, DeleteLabel, UpdateLabel, User, Response};
use crate::models::template::{Tab, LabelsTemplate, NewIssueTemplate, IssueTemplate, IssuesTemplate};

//##### Routes #####//
// ✔ [get]          /{user|org}/{project}/issues
// ✔ [get|post]     /{user|org}/{project}/issues/new
// ✔ [get|put]      /{user|org}/{project}/issues/{number}
// ✔ [post]         /{user|org}/{project}/issues/{number}/comments
// ✔ [get|delete]   /{user|org}/{project}/issues/labels
// ✔ [post]         /{user|org}/{project}/issues/labels/new
// ✔ [put]          /{user|org}/{project}/issues/labels/edit

pub fn get_routes() -> Vec<Route> {
    routes![
        issues_get,
        issue_new_get,
        issue_new_post,
        issue_get,
        issue_update,
        issue_comments_new_post,
        labels_get,
        labels_new_post,
        labels_edit_post,
        labels_delete,
    ]
}

#[get("/<owner>/<projectname>/issues")]
pub fn issues_get(logged_user: Option<User>, conn: DbConn, owner: String, projectname: String) -> Response<IssuesTemplate> {
    let owner = User::by_name(&conn, &owner)?;
    let project = owner.get_or_find_project(&conn, &projectname)?;
    let collaborators = project.all_collaborators(&conn)?.expect("No collaborators found");
    
    if project.is_private &&
       (logged_user.is_none() || logged_user.as_ref().unwrap() != &owner || !collaborators.contains(&logged_user.as_ref().unwrap())) {
        return Response::Redirect(Redirect::to(uri!(crate::routes::login_get: return_to=format!("/{}/{}/issues", owner.username, project.name))));
    }

    let issues = project.all_issues(&conn)?;

    Response::Ok(IssuesTemplate {
        debug: Config::global().debug,
        user: logged_user,
        project,
        issues,
        active_tab: Tab::Issues,
        branch: None,
    })
}

#[get("/<owner>/<projectname>/issues/new")]
pub fn issue_new_get(logged_user: Option<User>, conn: DbConn, owner: String, projectname: String) -> Response<NewIssueTemplate> {
    let owner = User::by_name(&conn, &owner)?;
    let project = owner.get_or_find_project(&conn, &projectname)?;
    let collaborators = project.all_collaborators(&conn)?.expect("No collaborators found");
    
    if project.is_private &&
       (logged_user.is_none() || logged_user.as_ref().unwrap() != &owner || !collaborators.contains(&logged_user.as_ref().unwrap())) {
        return Response::Redirect(Redirect::to(uri!(crate::routes::login_get: return_to=format!("/{}/{}/issues/new", owner.username, project.name))));
    }

    let labels = project.all_labels(&conn)?;
    //TODO: check if user is allowed to create issues

    Response::Ok(NewIssueTemplate {
        debug: Config::global().debug,
        user: logged_user,
        project,
        active_tab: Tab::Issues,
        branch: None,
        active_labels: None,
        active_assignees: None,
        labels,
        collaborators,
    })
}

#[post("/<owner>/<projectname>/issues/new", data = "<new_issue>")]
pub fn issue_new_post(logged_user: Option<User>, conn: DbConn, owner: String, projectname: String, new_issue: Form<NewIssue>) -> Response<()> {
    let owner = User::by_name(&conn, &owner)?;
    let project = owner.get_or_find_project(&conn, &projectname)?;
    let collaborators = project.all_collaborators(&conn)?.expect("No collaborators found");
    
    if project.is_private &&
       (logged_user.is_none() || logged_user.as_ref().unwrap() != &owner || !collaborators.contains(&logged_user.as_ref().unwrap())) {
        return Response::Redirect(Redirect::to(uri!(crate::routes::login_get: return_to=format!("/{}/{}/issues/new", owner.username, project.name))));
    }

    project.new_issue(&conn, &new_issue)?;

    Response::Redirect(Redirect::to(uri!(issues_get: owner = &owner.username, projectname = project.name)))
}

#[get("/<owner>/<projectname>/issues/<issue_num>", rank = 2)]
pub fn issue_get(logged_user: Option<User>, conn: DbConn, owner: String, projectname: String, issue_num: u32) -> Response<IssueTemplate> {
    let owner = User::by_name(&conn, &owner)?;
    let project = owner.get_or_find_project(&conn, &projectname)?;
    let collaborators = project.all_collaborators(&conn)?.expect("No collaborators found");
    
    if project.is_private &&
       (logged_user.is_none() || logged_user.as_ref().unwrap() != &owner || !collaborators.contains(&logged_user.as_ref().unwrap())) {
        return Response::Redirect(Redirect::to(uri!(crate::routes::login_get: return_to=format!("/{}/{}/issues/{}", owner.username, project.name, issue_num))));
    }
    
    let issue = project.issue_by_number(&conn, issue_num as i32)?;
    let labels = project.all_labels(&conn)?;

    Response::Ok(IssueTemplate {
        debug: Config::global().debug,
        user: logged_user,
        project,
        active_tab: Tab::Issues,
        branch: None,
        active_labels: issue.labels.as_ref().map(|ls| ls.iter().map(|l| l.id).collect::<Vec<i32>>()),
        active_assignees: issue.assignees.as_ref().map(|asgs| asgs.iter().map(|a| a.id).collect::<Vec<i32>>()),
        issue,
        labels,
        collaborators,
    })
}

#[put("/<owner>/<projectname>/issues/<issue_num>", data = "<updated_issue>")]
pub fn issue_update(logged_user: Option<User>, conn: DbConn, owner: String, projectname: String, issue_num: u32, updated_issue: Form<UpdateIssue>) -> Response<()> {
    let owner = User::by_name(&conn, &owner)?;
    let project = owner.get_or_find_project(&conn, &projectname)?;
    let collaborators = project.all_collaborators(&conn)?.expect("No collaborators found");
    
    if project.is_private &&
       (logged_user.is_none() || logged_user.as_ref().unwrap() != &owner || !collaborators.contains(&logged_user.as_ref().unwrap())) {
        return Response::Redirect(Redirect::to(uri!(crate::routes::login_get: return_to=format!("/{}/{}/issues/{}", owner.username, project.name, issue_num))));
    }

    project.update_issue_number(&conn, issue_num as i32, &updated_issue)?;

    Response::Redirect(Redirect::to(uri!(issue_get: owner = &owner.username, projectname = project.name, issue_num = issue_num)))
}

#[post("/<owner>/<projectname>/issues/<issue_num>/comments", data = "<new_comment>")]
pub fn issue_comments_new_post(logged_user: Option<User>, conn: DbConn, owner: String, projectname: String, issue_num: u32, new_comment: Form<NewComment>) -> Response<()> {
    let owner = User::by_name(&conn, &owner)?;
    let project = owner.get_or_find_project(&conn, &projectname)?;
    let collaborators = project.all_collaborators(&conn)?.expect("No collaborators found");
    
    if project.is_private &&
       (logged_user.is_none() || logged_user.as_ref().unwrap() != &owner || !collaborators.contains(&logged_user.as_ref().unwrap())) {
        return Response::Redirect(Redirect::to(uri!(crate::routes::login_get: return_to=format!("/{}/{}/issues/{}/comment", owner.username, project.name, issue_num))));
    }

    project.new_comment_for_issue_number(&conn, issue_num as i32, &new_comment)?;
    
    Response::Redirect(Redirect::to(uri!(issue_get: owner = &owner.username, projectname = project.name, issue_num = issue_num)))
}

#[get("/<owner>/<projectname>/issues/labels")]
pub fn labels_get(logged_user: Option<User>, conn: DbConn, owner: String, projectname: String) -> Response<LabelsTemplate> {
    let owner = User::by_name(&conn, &owner)?;
    let project = owner.get_or_find_project(&conn, &projectname)?;
    let collaborators = project.all_collaborators(&conn)?.expect("No collaborators found");
    
    if project.is_private &&
       (logged_user.is_none() || logged_user.as_ref().unwrap() != &owner || !collaborators.contains(&logged_user.as_ref().unwrap())) {
        return Response::Redirect(Redirect::to(uri!(crate::routes::login_get: return_to=format!("/{}/{}/issues/labels", owner.username, project.name))));
    }
    
    let labels = project.all_labels(&conn)?;
    //TODO: check if user is allowed to create issues

    Response::Ok(LabelsTemplate {
        debug: Config::global().debug,
        user: logged_user,
        project,
        active_tab: Tab::Issues,
        branch: None,
        labels,
    })
}

#[delete("/<owner>/<projectname>/issues/labels", data = "<delete_label>")]
pub fn labels_delete(logged_user: Option<User>, conn: DbConn, owner: String, projectname: String, delete_label: LenientForm<DeleteLabel>) -> Response<()> {
    let owner = User::by_name(&conn, &owner)?;
    let project = owner.get_or_find_project(&conn, &projectname)?;
    let collaborators = project.all_collaborators(&conn)?.expect("No collaborators found");
    
    if project.is_private &&
       (logged_user.is_none() || logged_user.as_ref().unwrap() != &owner || !collaborators.contains(&logged_user.as_ref().unwrap())) {
        return Response::Redirect(Redirect::to(uri!(crate::routes::login_get: return_to=format!("/{}/{}/issues/labels", owner.username, project.name))));
    }

    delete_label.delete(&conn)?;

    Response::Redirect(Redirect::to(uri!(labels_get: owner = &owner.username, projectname = project.name)))
}

#[post("/<owner>/<projectname>/issues/labels/new", data = "<new_label>")]
pub fn labels_new_post(logged_user: Option<User>, conn: DbConn, owner: String, projectname: String, new_label: LenientForm<NewLabel>) -> Response<()> {
    let owner = User::by_name(&conn, &owner)?;
    let project = owner.get_or_find_project(&conn, &projectname)?;
    let collaborators = project.all_collaborators(&conn)?.expect("No collaborators found");
    
    if project.is_private &&
       (logged_user.is_none() || logged_user.as_ref().unwrap() != &owner || !collaborators.contains(&logged_user.as_ref().unwrap())) {
        return Response::Redirect(Redirect::to(uri!(crate::routes::login_get: return_to=format!("/{}/{}/issues/labels", owner.username, project.name))));
    }

    project.new_label(&conn, &new_label)?;

    Response::Redirect(Redirect::to(uri!(labels_get: owner = &owner.username, projectname = project.name)))
}

#[put("/<owner>/<projectname>/issues/labels/edit", data = "<update_label>")]
pub fn labels_edit_post(logged_user: Option<User>, conn: DbConn, owner: String, projectname: String, update_label: LenientForm<UpdateLabel>) -> Response<()> {
    let owner = User::by_name(&conn, &owner)?;
    let project = owner.get_or_find_project(&conn, &projectname)?;
    let collaborators = project.all_collaborators(&conn)?.expect("No collaborators found");
    
    if project.is_private &&
       (logged_user.is_none() || logged_user.as_ref().unwrap() != &owner || !collaborators.contains(&logged_user.as_ref().unwrap())) {
        return Response::Redirect(Redirect::to(uri!(crate::routes::login_get: return_to=format!("/{}/{}/issues/labels", owner.username, project.name))));
    }

    update_label.update(&conn)?;

    Response::Redirect(Redirect::to(uri!(labels_get: owner = &owner.username, projectname = project.name)))
}