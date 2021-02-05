#![feature(proc_macro_hygiene, decl_macro)]
#![feature(crate_visibility_modifier)]
#![feature(type_alias_impl_trait)]
#![feature(option_result_contains)]
#![feature(exclusive_range_pattern)]
#![feature(try_trait)]
#![warn(clippy::all)]
#![recursion_limit="128"]
#![warn(rust_2018_idioms)]
#![allow(unused_extern_crates)]

#[macro_use] extern crate anyhow;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;
#[macro_use] extern crate log;
//use diesel_migrations;
use once_cell::sync::OnceCell;
use rocket_contrib::serve::{StaticFiles};
use diesel::sqlite::SqliteConnection;
use models::Config;

pub mod db;
pub mod models;
pub mod routes;
pub mod utils;
pub mod vcs;

#[database("sqlite_zorgit")]
pub struct DbConn(SqliteConnection);
static CONFIG: OnceCell<Config> = OnceCell::new();

fn main() {
    load_config();
    rocket::custom(Config::global().rocket_config())
        .attach(DbConn::fairing())
        .mount("/", routes::get_routes())
        .mount("/static", StaticFiles::from("static").rank(11))
        .mount("/static/img/", StaticFiles::from("assets/Logos"))
        .mount("/avatars", StaticFiles::from(Config::global().avatars_dir()))
        .register(routes::error_catchers::get_catchers())
        .launch();
}

fn load_config() {
    info!("Loading Zorgit config.");
    let config = Config::new();
    if let Err(msg) = config {
        panic!(msg);
    }
    let config = CONFIG.set(config.unwrap());
    if let Err(msg) = config {
        panic!(msg);
    }
}
