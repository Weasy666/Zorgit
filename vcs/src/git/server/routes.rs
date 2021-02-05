use crate::git::{self, server::GitContentType};
use rocket::data::Data;
use rocket::http::Status;
use rocket::Response;
use rocket::response::{status, ResponseBuilder};
use rocket::{get, post, routes};
use std::io;
use zorgit_common::{DotFile, Entity};

//##### Routes #####//
//#### Smart git protocol ####
// ✔ [get]     /{user|org}/{project}/info/refs?service=<...>
// ✔ [post]    /{user|org}/{project}/git-upload-pack
// ✔ [post]    /{user|org}/{project}/git-receive-pack
//#### Dumb git protocol ####
// ⛔ [get]     /{user|org}/{project}/HEAD
// ⛔ [get]     /{user|org}/{project}/objects/info/alternate
// ⛔ [get]     /{user|org}/{project}/objects/info/http-alternates
// ⛔ [get]     /{user|org}/{project}/objects/info/packs
// ⛔ [get]     /{user|org}/{project}/objects/info/<requested>
// ⛔ [get]     /{user|org}/{project}/objects/<hex_2>/<hex_38>
// ⛔ [get]     /{user|org}/{project}/objects/pack/<pack_sha>.pack
// ⛔ [get]     /{user|org}/{project}/objects/pack/<pack_sha>.idx

pub struct Server;

impl crate::vcs::Server for Server {
    const EXT: &'static str = "git";
    const ROUTES: &'static [rocket::Route] = routes![
        info_refs_get,
        upload_pack_post,
        receive_pack_post,
        head_get,
        info_alt_get,
        info_http_alt_get,
        info_packs_get,
        info_all_get,
        loose_object,
        pack_get,
    ].as_ref();
}


trait GitResponse<'r> {
    fn git_headers(&mut self) -> &mut ResponseBuilder<'r>;
    fn unauthorized(&mut self) -> &mut ResponseBuilder<'r>;
}

impl<'r> GitResponse<'r> for ResponseBuilder<'r> {
    fn git_headers(&mut self) -> &mut ResponseBuilder<'r> {
        self.raw_header("Expires", "Fri, 01 Jan 1980 00:00:00 GMT")
            .raw_header("Pragma", "no-cache")
            .raw_header("Cache-Control", "no-cache, max-age=0, must-revalidate")
    }

    fn unauthorized(&mut self) -> &mut ResponseBuilder<'r> {
        self.status(Status::Unauthorized)
            .git_headers()
            .raw_header("WWW-Authenticate", r#"Basic realm="Zorgit", charset="UTF-8""#)
    }
}


//################# Smart git protocol #################
#[get("/<owner>/<project_git>/info/refs?<service>")]
pub fn info_refs_get(logged_user: Option<Entity>, owner: Entity, project_git: DotFile, service: String) -> Response<'static> {
    let project = owner.get_or_find_project(&conn, &project_git.file_stem().unwrap().to_str().unwrap()).expect("Could not find project");
    let collaborators = project.all_collaborators(&conn).expect("Could not load collaborators").expect("No collaborators found");

    if project.is_private &&
       (logged_user.is_none() || logged_user.as_ref().unwrap() != &owner || !collaborators.contains(&logged_user.as_ref().unwrap())) {
        return Response::build().unauthorized().finalize();
    }

    let service = service.replace("git-", "");
    let data = git::server::info_refs(project.dir(), &service);

    match data {
        Ok(data) => Response::build()
                        .git_headers()
                        .header(GitContentType::ADVERTISEMENT(service.into()))
                        .sized_body(data.len(), io::Cursor::new(data))
                        .finalize(),
        Err(_) => Response::build().status(Status::NotFound).finalize(),
    }
}

#[post("/<owner>/<project_git>/git-upload-pack", data = "<data>")]
pub fn upload_pack_post(logged_user: Option<User>, owner: Entity, project_git: DotFile, data: Data) -> Response<'static> {
    let project = owner.get_or_find_project(&conn, &project_git.file_stem().unwrap().to_str().unwrap()).expect("Could not find project");
    let collaborators = project.all_collaborators(&conn).expect("Could not load collaborators").expect("No collaborators found");

    if project.is_private &&
       (logged_user.is_none() || logged_user.as_ref().unwrap() != &owner || !collaborators.contains(&logged_user.as_ref().unwrap())) {
        return Response::build().unauthorized().finalize();
    }

    let data = git::server::upload_pack(project.dir(), data);

    match data {
        Ok(data) => Response::build()
                        .git_headers()
                        .header(GitContentType::UPLOAD_PACK)
                        .sized_body(data.len(), io::Cursor::new(data))
                        .finalize(),
        Err(_) => Response::build().status(Status::NotFound).finalize(),
    }
}

#[post("/<owner>/<project_git>/git-receive-pack", data = "<data>")]
pub fn receive_pack_post(logged_user: Option<User>, owner: Entity, project_git: DotFile, data: Data) -> Response<'static> {
    let owner = User::by_name(&conn, &owner).expect("Owner not found");
    let project = owner.get_or_find_project(&conn, &project_git.file_stem().unwrap().to_str().unwrap()).expect("Could not find project");
    let collaborators = project.all_collaborators(&conn).expect("Could not load collaborators").expect("No collaborators found");

    if project.is_private &&
       (logged_user.is_none() || logged_user.as_ref().unwrap() != &owner || !collaborators.contains(&logged_user.as_ref().unwrap())) {
        return Response::build().unauthorized().finalize();
    }

    if project.is_empty {
        project.init(&conn).unwrap();
    }

    let data = git::server::resceive_pack(project.dir(), data.open(limit));

    match data {
        Ok(data) => Response::build()
                        .git_headers()
                        .header(GitContentType::RECEIVE_PACK)
                        .sized_body(data.len(), io::Cursor::new(data))
                        .finalize(),
        Err(_) => Response::build().status(Status::NotFound).finalize(),
    }
}

//################# Dumb git protocol #################
const NO_DUMB_GIT_MSG: &str = "This server does not support the dumb git protocol. Please use the smart git protocol.";

#[get("/<_owner>/<_project_git>/HEAD")]
pub fn head_get(_owner: String, _project_git: DotFile) -> status::Custom<&'static str> {
    status::Custom(Status::Forbidden, NO_DUMB_GIT_MSG)
}

#[get("/<_owner>/<_project_git>/objects/info/alternates")]
pub fn info_alt_get(_owner: String, _project_git: DotFile) -> status::Custom<&'static str> {
    status::Custom(Status::Forbidden, NO_DUMB_GIT_MSG)
}

#[get("/<_owner>/<_project_git>/objects/info/http-alternates")]
pub fn info_http_alt_get(_owner: String, _project_git: DotFile) -> status::Custom<&'static str> {
    status::Custom(Status::Forbidden, NO_DUMB_GIT_MSG)
}

#[get("/<_owner>/<_project_git>/objects/info/packs")]
pub fn info_packs_get(_owner: String, _project_git: DotFile) -> status::Custom<&'static str> {
    status::Custom(Status::Forbidden, NO_DUMB_GIT_MSG)
}

#[get("/<_owner>/<_project_git>/objects/info/<_requested>", rank = 20)]
pub fn info_all_get(_owner: String, _project_git: DotFile, _requested: String) -> status::Custom<&'static str> {
    status::Custom(Status::Forbidden, NO_DUMB_GIT_MSG)
}

#[get("/<_owner>/<_project_git>/<_hex_2>/<_hex_38>", rank = 20)]
pub fn loose_object(_owner: String, _project_git: DotFile, _hex_2: String, _hex_38: String) -> status::Custom<&'static str> {
    status::Custom(Status::Forbidden, NO_DUMB_GIT_MSG)
}

#[get("/<_owner>/<_project_git>/objects/pack/<_pack_sha>")] // .idx and .pack
pub fn pack_get(_owner: String, _project_git: DotFile, _pack_sha: DotFile) -> status::Custom<&'static str> {
    status::Custom(Status::Forbidden, NO_DUMB_GIT_MSG)
}
