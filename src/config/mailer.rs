use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use zorgit_common::Url;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename = "mailer")]
pub struct Config {
    /// Activates the mailer service of zorgit with the selected mailer service.
    /// If no service is selected than the mailer is disabled and zorgit won't send
    /// any confirmation or notification emails.
    //pub service: Option<MailerService>,
    /// Indicates if a HELO message is send.
    pub with_helo: bool,
    /// Can be used to define a custom hostname for HELO, otherwise the server address will be used.
    pub helo_hostname: Option<String>,
    /// Address of the SMTP server over which emails will be send.
    pub smtp_server: Option<Url>,
    /// User for authentication with SMTP server.
    pub smtp_user: Option<String>,
    /// Password for authentication with SMTP server.
    pub smtp_password: Option<String>,
    /// Custom name/email that the recepient will see as sender.
    pub from: Option<String>,
    /// This will be prefixed before every sent emails subject line.
    pub subject_prefix: Option<String>,
    /// Uses the given path to look for sendmail.
    pub sendmail_path: Option<PathBuf>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            //service: None,
            with_helo: false,
            helo_hostname: None,
            smtp_server: None,
            smtp_user: None,
            smtp_password: None,
            from: None,
            subject_prefix: Some("[Zorgit]".to_string()),
            sendmail_path: None
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum MailerService {
    //Integrated, // Was initially there because i wanted to integrate Mailstrom, but Mailstrom only works on Linux
    SMTP,
    Sendmail,
}
