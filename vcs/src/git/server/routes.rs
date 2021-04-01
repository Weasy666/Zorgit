use crate::git::{self, server::Service};
use rocket::{
    get, post, routes, Response, Route,
    data::{Data, ToByteUnit},
    http::Status,
    response::{status, ResponseBuilder}
};
use std::io;
use zorgit_common::{
    Project, Sha1,
    entities::{Owner, User}
};

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

const UPLOAD_LIMIT: i32 = 20; //TODO: change to sensible limit

pub struct Server;

impl crate::vcs::Server for Server {
    const EXT: &'static str = "git";
    fn routes() -> Vec<Route> {
        routes![
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
        ]
    }
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
#[get("/<_owner>/<_project_name>/info/refs?<service>")]
pub async fn info_refs_get(_owner: Owner, _project_name: &str, project: Project, service: Service, logged_user: Option<User>) -> Response<'static> {
    if project.is_private {
        return Response::build().unauthorized().finalize();
    }

    let data = git::server::info_refs(project.dir, &service).await;

    match data {
        Ok(data) => Response::build()
                        .git_headers()
                        .header(service)
                        .sized_body(data.len(), io::Cursor::new(data))
                        .finalize(),
        Err(_) => Response::build().status(Status::NotFound).finalize(),
    }
}

#[post("/<_owner>/<_project_name>/git-upload-pack", data = "<data>")]
pub async fn upload_pack_post(_owner: Owner, _project_name: &str, project: Project, data: Data, _logged_user: User) -> Response<'static> {
    let data = git::server::upload_pack(project.dir, data.open(UPLOAD_LIMIT.mebibytes())).await;

    match data {
        Ok(data) => Response::build()
                        .git_headers()
                        .header(Service::UploadPack)
                        .sized_body(data.len(), io::Cursor::new(data))
                        .finalize(),
        Err(_) => Response::build().status(Status::NotFound).finalize(),
    }
}

#[post("/<_owner>/<_project_name>/git-receive-pack", data = "<data>")]
pub async fn receive_pack_post(_owner: Owner, _project_name: &str, project: Project, data: Data, _logged_user: User) -> Response<'static> {
    let data = git::server::receive_pack(project.dir, data.open(UPLOAD_LIMIT.mebibytes())).await;

    match data {
        Ok(data) => Response::build()
                        .git_headers()
                        .header(Service::ReceivePack)
                        .sized_body(data.len(), io::Cursor::new(data))
                        .finalize(),
        Err(_) => Response::build().status(Status::NotFound).finalize(),
    }
}

//################# Dumb git protocol #################
const NO_DUMB_GIT_MSG: &str = "This server does not support the dumb git protocol. Please use the smart git protocol.";

#[get("/<_owner>/<_project_name>/HEAD")]
pub fn head_get(_owner: Owner, _project_name: &str, _project: Project) -> status::Custom<&'static str> {
    status::Custom(Status::Forbidden, NO_DUMB_GIT_MSG)
}

#[get("/<_owner>/<_project_name>/objects/info/alternates")]
pub fn info_alt_get(_owner: Owner, _project_name: &str, _project: Project) -> status::Custom<&'static str> {
    status::Custom(Status::Forbidden, NO_DUMB_GIT_MSG)
}

#[get("/<_owner>/<_project_name>/objects/info/http-alternates")]
pub fn info_http_alt_get(_owner: Owner, _project_name: &str, _project: Project) -> status::Custom<&'static str> {
    status::Custom(Status::Forbidden, NO_DUMB_GIT_MSG)
}

#[get("/<_owner>/<_project_name>/objects/info/packs")]
pub fn info_packs_get(_owner: Owner, _project_name: &str, _project: Project) -> status::Custom<&'static str> {
    status::Custom(Status::Forbidden, NO_DUMB_GIT_MSG)
}

#[get("/<_owner>/<_project_name>/objects/info/<_requested>", rank = 20)]
pub fn info_all_get(_owner: Owner, _project_name: &str, _project: Project, _requested: String) -> status::Custom<&'static str> {
    status::Custom(Status::Forbidden, NO_DUMB_GIT_MSG)
}

#[get("/<_owner>/<_project_name>/<_hex_2>/<_hex_38>", rank = 20)]
pub fn loose_object(_owner: Owner, _project_name: &str, _project: Project, _hex_2: String, _hex_38: String) -> status::Custom<&'static str> {
    status::Custom(Status::Forbidden, NO_DUMB_GIT_MSG)
}

#[get("/<_owner>/<_project_name>/objects/pack/<_pack_sha>")] // .idx and .pack
pub fn pack_get(_owner: Owner, _project_name: &str, _project: Project, _pack_sha: Sha1) -> status::Custom<&'static str> {
    status::Custom(Status::Forbidden, NO_DUMB_GIT_MSG)
}
