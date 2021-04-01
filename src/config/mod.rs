use std::path::{Path, PathBuf};
use std::str::FromStr;
use directories::{ProjectDirs, UserDirs};
use mailer::Config as MailerConfig;
use rocket::{
    error,
    config::{Config, SecretKey},
    fairing::AdHoc,
    figment::{Figment, providers::{Format, Toml, Serialized, Env}},
};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use zorgit_common::Url;

pub mod mailer;


#[derive(Debug, Deserialize, Serialize)]
pub struct ZorgitConfig {
    /// The server domain. This is used for generating links and for configuration of Rocket.
    pub domain: Url,
    #[serde(skip_serializing)]
    /// Secret key that is used as seed
    pub secret_key: SecretKey,
    /// Name with which the session value will be added to a cookie
    pub session_key: String,
    /// Duration of a login session in days
    pub session_duration: u64,
    /// Folder in which all data (like attachements, avatars) is stored
    pub data_path: PathBuf,
    /// Struct for project specific values
    pub projects: Projects,
    /// All attachment related configs
    pub attachments: Attachments,
    /// Holds the configuration for the Mailer service.
    pub mailer: MailerConfig,
}

impl Default for ZorgitConfig {
    fn default() -> Self {
        ZorgitConfig {
            domain: Url::from_str("http://127.0.0.1:2020").unwrap(),
            secret_key: SecretKey::from(&[0; 64]),
            session_key: "from_zorgit_with_❤".to_string(),
            session_duration: 7,
            data_path: project_dirs().data_dir().to_path_buf(),
            projects: Projects::default(),
            attachments: Attachments::default(),
            mailer: MailerConfig::default(),
        }
    }
}

impl ZorgitConfig {
    pub fn attach() -> AdHoc {
        AdHoc::on_attach("Config", |rocket| async {
            let app_config = match rocket.figment().extract::<Self>() {
                Ok(config) => config,
                Err(e) => {
                    rocket::config::pretty_print_error(e);
                    return Err(rocket);
                }
            };

            Ok(rocket.manage(app_config))
        })
    }

    pub fn figment() -> Figment {
        dotenv::dotenv().ok();

        let figment = rocket::Config::figment()
            .merge(Serialized::defaults(ZorgitConfig::default()))
            .merge(Toml::file(Env::var_or("ZORGIT_CONFIG", "Zorgit.toml")).nested())
            .merge(Env::prefixed("ZORGIT_").ignore(&["PROFILE"]).global());

        let mut zorgit_config = figment.extract::<Self>().unwrap_or_else(|e| {
            rocket::config::pretty_print_error(e);
            panic!("aborting due to configuration error(s)")
        });

        zorgit_config.data_path = absolutify(&zorgit_config.data_path);
        zorgit_config.projects.path = absolutify(&zorgit_config.projects.path);
        zorgit_config.attachments.path = absolutify(&zorgit_config.attachments.path);
        zorgit_config.attachments.avatars = absolutify(&zorgit_config.attachments.avatars);

        Self::create_all_dirs(&zorgit_config).unwrap_or_else(|e| {
            error!("{}", e);
            panic!("aborting due to dir creation error(s)")
        });

        #[cfg(debug_assertions)] { figment.merge(Serialized::from(zorgit_config, Config::DEBUG_PROFILE)) }
        #[cfg(not(debug_assertions))] { figment.merge(Serialized::from(zorgit_config, Config::RELEASE_PROFILE)) }
    }

    fn create_all_dirs(zorgit_config: &ZorgitConfig) -> std::io::Result<()> {
        std::fs::create_dir_all(&zorgit_config.data_path)?;
        std::fs::create_dir_all(&zorgit_config.projects.path)?;
        std::fs::create_dir_all(&zorgit_config.attachments.path)?;
        std::fs::create_dir_all(&zorgit_config.attachments.avatars)
    }

    pub fn project_temp_dir(project_name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(&format!("Temp/zorgit_{}_{}", project_name, OffsetDateTime::now_utc().unix_timestamp()));
        std::fs::create_dir_all(&dir).unwrap_or_else(|e| {
            error!("{}", e);
            panic!("aborting due to dir creation error(s)")
        });
        dir
    }
}

fn absolutify<P: AsRef<Path>>(path: P) -> PathBuf {
    if path.as_ref().starts_with("~") {
        let stripped_path = path.as_ref().strip_prefix("~").expect("Prefix '~' couldn't be stripped from path");
        user_dirs().home_dir().join(stripped_path)
    } else if path.as_ref().is_relative() {
        project_dirs().data_dir().join(path)
    } else {
        path.as_ref().to_path_buf()
    }
}

const DIRECTORIES_MSG: &str = "No valid home directory path could be retrieved from the operating system.";

fn project_dirs() -> ProjectDirs {
    ProjectDirs::from("eu", "", "Zorgit").expect(DIRECTORIES_MSG)
}

fn user_dirs() -> UserDirs {
    UserDirs::new().expect(DIRECTORIES_MSG)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Attachments {
    /// Path to attachments root folder
    pub path: PathBuf,
    /// Folder in which all avatars will be stored. It is placed inside of the data folder
    pub avatars: PathBuf,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Projects {
    /// Path to the folder in which all projects are stored
    pub path: PathBuf,
    /// Struct for pull request specific values
    pub pull_requests: PullRequests,
}

impl Default for Projects {
    fn default() -> Self {
        Projects {
            path: project_dirs().data_dir().join("zorgit-projects"),
            pull_requests: PullRequests::default(),
        }
    }
}

impl Default for Attachments {
    fn default() -> Self {
        Attachments {
            path: project_dirs().data_dir().join("attachments"),
            avatars: project_dirs().data_dir().join("avatars")
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PullRequests {
    /// The start of a pull request title will be matched for this values to determine if its a work in progress
    pub work_in_progress_prefixes: Vec<String>,
}

impl Default for PullRequests {
    fn default() -> Self {
        PullRequests {
            work_in_progress_prefixes: vec!["WIP:".to_string(), "[WIP]:".to_string()]
        }
    }
}
