#![warn(rust_2018_idioms)]
use std::path::PathBuf;
use rocket_contrib::serve::StaticFiles;
use zorgit_security::{AuthHatch, rocket_airlock::Airlock};
use crate::config::ZorgitConfig;
use zorgit_common::Url;
use zorgit_vcs;

mod config;


#[rocket::launch]
fn rocket() -> _ {
    let figment = ZorgitConfig::figment();

    let avatars = figment.extract_inner::<PathBuf>("attachments.avatars").expect("Avatars path");
    rocket::custom(figment)
        .mount("/static", StaticFiles::from("static").rank(11))
        .mount("/static/img/", StaticFiles::from("assets/Logos"))
        .mount("/avatars", StaticFiles::from(avatars))
        .attach(ZorgitConfig::attach())
        .attach(Airlock::<AuthHatch>::fairing())
}
