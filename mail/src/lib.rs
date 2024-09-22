use std::io::BufReader;

use lettre::{
    message::Mailbox,
    transport::smtp::{
        authentication::{Credentials, Mechanism},
        client::TlsParametersBuilder,
    },
    Message, SmtpTransport, Transport,
};

#[derive(Debug)]
pub struct MailService {
    from_address: String,
    smtp_url: String,
    smtp_port: u16,
    user: String,
    password: String,
}

pub fn get_service_from_env() -> Result<MailService, Box<dyn std::error::Error>> {
    Ok(MailService {
        from_address: std::env::var("EMAIL_FROM_ADDRESS")?,
        smtp_url: std::env::var("EMAIL_SMTP_URL")?,
        smtp_port: std::env::var("EMAIL_SMTP_PORT")?.parse()?,
        user: std::env::var("EMAIL_USER")?,
        password: std::env::var("EMAIL_PASSWORD")?,
    })
}

fn get_transport(service: &MailService) -> Result<SmtpTransport, Box<dyn std::error::Error>> {
    let tls_parameters = TlsParametersBuilder::new(service.smtp_url.to_string())
        .set_min_tls_version(lettre::transport::smtp::client::TlsVersion::Tlsv12)
        .build()
        .expect("Should be able to build parameters.");

    // Create TLS transport on port 587
    let sender = SmtpTransport::relay(&service.smtp_url)?
        .port(service.smtp_port)
        .credentials(Credentials::new(
            service.user.to_string(),
            service.password.to_string(),
        ))
        .tls(lettre::transport::smtp::client::Tls::Required(
            tls_parameters,
        ))
        .build();

    Ok(sender)
}

pub fn send_mail(
    service: &MailService,
    recipient: &str,
    subject: String,
    body: String,
) -> Result<lettre::transport::smtp::response::Response, Box<dyn std::error::Error>> {
    let sender: Mailbox = service.from_address.parse()?;

    if !is_valid_email(recipient) {
        return Err("Invalid e-mail address.".into());
    }

    let email = Message::builder()
        .from(sender.clone())
        .reply_to(sender)
        .to(recipient.parse()?)
        .subject(subject)
        .body(body)?;

    let sender = get_transport(service)?;

    Ok(sender.send(&email)?)
}

pub fn is_valid_email(address: &str) -> bool {
    validator::ValidateEmail::validate_email(&address)
}

mod tests {
    use super::*;

    #[test]
    fn test_secret() {
        dotenvy::dotenv().expect("Couldn't load environment variables.");
        get_service_from_env().expect("Couldn't get mail service.");
    }

    #[test]
    fn test_email_connectivity() {
        dotenvy::dotenv().expect("Couldn't load environment variables.");
        let service = get_service_from_env().expect("Couldn't get service.");
        let sender = get_transport(&service).expect("Couldnt get transport.");
        sender.test_connection().expect("Connection test failed.");
    }

    #[test]
    fn test_self_email_send() {
        dotenvy::dotenv().expect("Couldn't load environment variables.");
        let service = get_service_from_env().expect("Couldn't get service.");
        send_mail(
            &service,
            &service.from_address,
            "Block Divider E-mail Test".to_string(),
            "This is a test of programmatic e-mailing.".to_string(),
        )
        .expect("Couldn't send e-mail.");
    }
}
