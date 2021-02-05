use lettre::{SmtpClient, SmtpTransport, Transport, ClientSecurity, ClientTlsParameters};
use lettre_email::Email;
use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::smtp::extension::ClientId;
use native_tls::TlsConnector;
use anyhow::Result;
use crate::models::{self, Config, Sha1};


pub struct Mailer {

}

impl Mailer {
    pub fn create_smtp() -> Result<SmtpTransport> {
        let server = Config::global().mailer().smtp_server().unwrap();

        let tls_builder = TlsConnector::builder().build().unwrap();
        let tls_parameters = ClientTlsParameters::new(server.scheme().to_string(), tls_builder);

        // Open a connection
        let mut mailer = SmtpClient::new(&server.to_string(), ClientSecurity::Opportunistic(tls_parameters))?
            // Add credentials for authentication
            .credentials(Credentials::new(Config::global().mailer().smtp_login().unwrap(), Config::global().mailer().smtp_password().unwrap()))
            // Enable SMTPUTF8 if the server supports it
            .smtp_utf8(true)
            // Configure expected authentication mechanism
            .authentication_mechanism(Mechanism::Login);
        if let Some(hostname) = Config::global().mailer().helo_hostname() {
            // Set the name sent during EHLO/HELO, default is `localhost`
            mailer = mailer.hello_name(ClientId::Domain(hostname));
        }

        Ok(mailer.transport())
    }

    pub fn send_confirm_user(user: &models::User, token: &Sha1) -> Result<()> {
        let confirm_url =  Config::global().root_url().join(&format!("{}/confirm/{}", user.username, token))?;
        let email = Email::builder()
            .to(user.email.address.as_str())
            .from(Config::global().mailer().from().unwrap_or("info@zorgit.com".to_string()))
            .subject(format!("{} Please confirm your account", Config::global().mailer().subject_prefix().unwrap_or("[Zorgit]".to_string())))
            .alternative(format!("<h2>Hello {}! Welcome to Zorgit!</h2></br>Please click the
                link below to verify your email address and confirm your account. Thank you!</br>
                {}", &user.username, confirm_url),
                format!("Hello {}! Welcome to Zorgit!\nPlease click the
                link below to verify your email address and confirm your account. Thank you!\n
                {}", &user.username, confirm_url)
            )
            .build()?;
        
        let mut smtp_mailer = Mailer::create_smtp()?;
        smtp_mailer.send(email.into())?;
        Ok(())
    }

    pub fn send_confirm_additional_email(user: &models::User, email: &models::Email, token: &Sha1) -> Result<()> {
        let confirm_url =  Config::global().root_url().join(&format!("{}/confirm/{}", user.username, token))?;
        let email = Email::builder()
            .to(email.address.as_str())
            .from(Config::global().mailer().from().unwrap_or("info@zorgit.com".to_string()))
            .subject(format!("{} Please confirm your additional email address", Config::global().mailer().subject_prefix().unwrap_or("[Zorgit]".to_string())))
            .alternative(format!("<h2>Hello {}! Welcome to Zorgit!</h2></br>Please click the
                link below to verify your additional email address. Thank you!</br>
                {}", &user.username, confirm_url),
                format!("Hello {}! Welcome to Zorgit!\nPlease click the
                link below to verify your email address. Thank you!\n
                {}", &user.username, confirm_url)
            )
            .build()?;
        
        let mut smtp_mailer = Mailer::create_smtp()?;
        smtp_mailer.send(email.into())?;
        Ok(())
    }

    pub fn send_test_smtp() -> Result<()> {
        let token = "TEST_TOKEN";
        let confirm_url =  Config::global().root_url().join("Weasy/confirm/")?.join(token)?;
        let email = Email::builder()
            .to("weasy666@gmail.com")
            .from("zorgit@outlook.de")
            .subject(format!("{} Please confirm your email address", Config::global().mailer().subject_prefix().unwrap_or("[Zorgit]".to_string())))
            .alternative(format!("<h2>Hello {}! Welcome to Zorgit!</h2></br>Please click the
                link below to verify your email address. Thank you!</br>
                {}", "Weasy", confirm_url),
                format!("Hello {}! Welcome to Zorgit!\nPlease click the
                link below to verify your email address. Thank you!\n
                {}", "Weasy", confirm_url)
            )
            .build()?;
        
        let mut smtp_mailer = Mailer::create_smtp()?;
        smtp_mailer.send(email.into())?;
        Ok(())
    }
}