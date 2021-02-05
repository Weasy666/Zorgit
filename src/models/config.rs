use crate::CONFIG;
use std::path::{Path, PathBuf};
use directories::{BaseDirs, UserDirs};
use config::{Config as Conf, File, ConfigError};
use serde::Deserialize;
use crate::models::Url;
use chrono::prelude::{Timelike, Utc};
use rocket::config::{Environment, Value};
use anyhow::Result;

#[derive(Debug, Deserialize)]
struct Projects {
    /// Path to the root folder in which all projects are stored
    path: PathBuf,
    /// Struct for pull request specific values
    pull_requests: PullRequests,
}

#[derive(Debug, Deserialize)]
struct PullRequests {
    /// The start of a pull request title will be matched for this values to determine if its a work in progress
    work_in_progress_prefixes: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Attachments {
    /// Path to attachments root folder
    path: PathBuf,
    /// All avatar related configs
    avatars: Avatars,
}

#[derive(Debug, Deserialize)]
struct Avatars {
    /// Folder in which all avatars will be stored. It is placed inside of the data folder
    path: PathBuf,
}

#[derive(Debug, Deserialize)]
struct Server {
    /// 
    cert_file: PathBuf,
    key_file: PathBuf,
    /// Address of the server. This is used for generating links and for configuration of Rocket.
    address: Url,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename = "mailer")]
pub struct MailerConfig {
    /// Activates the mailer service of zorgit with the selected mailer service.
    /// If no service is selected than the mailer is disabled and zorgit won't send
    /// any confirmation or notification emails.
    service: Option<MailerService>,
    /// Indicates if a HELO message is send.
    with_helo: bool,
    /// Can be used to define a custom hostname for HELO, otherwise the server address will be used.
    helo_hostname: Option<String>,
    /// Address of the SMTP server over which emails will be send.
    smtp_server: Option<Url>,
    /// Login for authentication with SMTP server.
    smtp_login: Option<String>,
    /// Password for authentication with SMTP server.
    smtp_password: Option<String>,
    /// Custom name/email that the recepient will see as sender.
    from: Option<String>,
    /// This will be prefixed before every sent emails subject line.
    subject_prefix: Option<String>,
    /// Uses the given path to look for sendmail.
    sendmail_path: Option<PathBuf>,
}

impl MailerConfig {
    pub fn is_enabled(&self) -> bool {
        self.service.is_some()
    }

    pub fn with_helo(&self) -> bool {
        self.with_helo
    }

    pub fn helo_hostname(&self) -> Option<String> {
        self.helo_hostname.clone()
    }
    
    pub fn smtp_server(&self) -> Option<Url> {
        self.smtp_server.clone()
    }
    
    pub fn smtp_login(&self) -> Option<String> {
        self.smtp_login.clone()
    }
    
    pub fn smtp_password(&self) -> Option<String> {
        self.smtp_password.clone()
    }
    
    pub fn from(&self) -> Option<String> {
        self.from.clone()
    }
    
    pub fn subject_prefix(&self) -> Option<String> {
        self.subject_prefix.clone()
    }
    
    pub fn sendmail_path(&self) -> Option<PathBuf> {
        self.sendmail_path.clone()
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename = "service")]
enum MailerService {
    //Integrated, // Was initially there because i wanted to integrate Mailstrom, but Mailstrom only works on Linux
    SMTP,
    Sendmail,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    /// This is for development
    pub debug: bool,
    /// Secret key that is used as seed
    secret_key: String,
    /// Name with which the session value will be added to a cookie
    session_key: String,
    /// Duration of a login session in days
    session_duration: u64,
    /// Folder in which all data (like attachements, avatars) are stored
    data_path: PathBuf,
    /// Server specific values
    server: Server,
    /// Struct for project specific values
    projects: Projects,
    /// All attachment related configs
    attachments: Attachments,
    /// All databases in a collection as (key = name, value = collection(key = url_name, value = url)).
    /// This is a bit strange but thats a toml thing.
    databases: std::collections::BTreeMap<String, std::collections::HashMap<String, String>>,
    /// Holds the configuration for the Mailer service.
    mailer: MailerConfig,
}

impl Config {
    pub fn new() -> Result<Config> {
        let config = Config::load_config_file("Zorgit.toml")?;
        Ok(config)
    }

    fn load_config_file(path: &str) -> Result<Self, ConfigError> {
        let mut zorgit_config = Conf::new();
        zorgit_config.merge(File::with_name(path))?
            .merge(File::with_name("Zorgit.Custom.toml"))?;

        let base_dirs = BaseDirs::new().unwrap();
        let user_dirs = UserDirs::new().unwrap();

        let mut zorgit_config: Config = zorgit_config.try_into()?;
        if zorgit_config.projects.path.starts_with("~") {
            let path = zorgit_config.projects.path.strip_prefix("~").unwrap();
            zorgit_config.projects.path = [user_dirs.home_dir(), path].iter().collect();
            Self::new_folder_if_not_exists(&zorgit_config.projects.path);
        }

        if zorgit_config.data_path.is_relative() {
            zorgit_config.data_path = [base_dirs.data_dir(), Path::new("Zorgit"), &zorgit_config.data_path].iter().collect();
            Self::new_folder_if_not_exists(&zorgit_config.data_path);
        }

        if zorgit_config.attachments.path.is_relative() {
            zorgit_config.attachments.path = [&zorgit_config.data_path, &zorgit_config.attachments.path].iter().collect();
            Self::new_folder_if_not_exists(&zorgit_config.attachments.path);
        }

        if zorgit_config.attachments.avatars.path.is_relative() {
            zorgit_config.attachments.avatars.path = [&zorgit_config.data_path, &zorgit_config.attachments.avatars.path].iter().collect();
            Self::new_folder_if_not_exists(&zorgit_config.attachments.avatars.path);
        }

        Ok(zorgit_config)
    }

    fn new_folder_if_not_exists(path: &PathBuf) -> &PathBuf {
        if !path.is_dir() {
            std::fs::create_dir_all(path).expect(&format!("Could not create path: {:#?}", path));
        }
        path
    }

    pub fn global() -> &'static Config {
        CONFIG.get().expect("Zorgit config is not initialized")
    }

    pub fn session_key(&self) -> String {
        self.session_key.to_owned()
    }

    pub fn session_duration(&self) -> u64 {
        self.session_duration.to_owned()
    }

    pub fn secret_key(&self) -> String {
        self.secret_key.to_owned()
    }

    pub fn rocket_config(&self) -> rocket::Config {
        let environment = if self.debug { Environment::Development } else { Environment::Production };

        let mut databases = std::collections::HashMap::new();
        for database in &self.databases {
            let database_config = database.1.to_owned();
            databases.insert(database.0, Value::from(database_config));
        }

        let builder = if self.server.address.scheme() == "https" {
            rocket::Config::build(environment)
                .tls(self.server.cert_file.to_str().unwrap(), self.server.key_file.to_str().unwrap())
        }
        else {
            rocket::Config::build(environment)
        };
        builder.address(self.server.address.host_str().unwrap())
            .port(self.server.address.port().unwrap())
            .secret_key(&self.secret_key)
            .extra("databases", databases)
            .finalize()
            .unwrap()
    }

    pub fn set_projects_dir(&mut self, path: &str) {
        self.projects.path = Self::new_folder_if_not_exists(&PathBuf::from(path)).into();
    }

    pub fn projects_dir(&self) -> PathBuf {
        Self::new_folder_if_not_exists(&self.projects.path).to_owned()
    }

    pub fn project_temp_dir(&self, project_name: &str) -> PathBuf {
        let dir = [BaseDirs::new().unwrap().cache_dir(), Path::new(&format!("Temp/zorgit_{}_{}", project_name, Utc::now().nanosecond()))].iter().collect();
        Self::new_folder_if_not_exists(&dir).to_owned()
    }

    pub fn data_dir(&self) -> PathBuf {
        self.data_path.to_owned()
    }

    pub fn attachments_dir(&self) -> PathBuf {
        self.attachments.path.to_owned()
    }

    pub fn avatars_dir(&self) -> PathBuf {
        self.attachments.avatars.path.to_owned()
    }

    pub fn root_url(&self) -> Url {
        self.server.address.to_owned()
    }

    pub fn mailer(&self) -> MailerConfig {
        self.mailer.clone()
    }
}
