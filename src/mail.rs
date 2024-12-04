use imap;
use native_tls;
use native_tls::TlsStream;
use std::net::TcpStream;
use std::env;
use lettre::message::{header::ContentType, header, Attachment, SinglePart, MultiPart, Message};
use dotenv::dotenv;
use crate::config_reader::{CompanyConfig, MailConfig};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MailError{
    #[error("IMAP error. Check credentials or connection.")]
    IMAPError(#[from] imap::error::Error),
    #[error("Password error")]
    PasswordError(#[from] env::VarError),
    #[error("Mail compose error")]
    ComposeError(#[from] lettre::error::Error),
    #[error("Content type error")]
    ContentTypeError(#[from] header::ContentTypeErr),
    #[error("TLS error")]
    TLSError(#[from] native_tls::Error)
}

type MailResult<T> = Result<T, MailError>;

fn get_imap_session(config: &MailConfig) -> MailResult<imap::Session<TlsStream<TcpStream>>> {
    dotenv().ok();
    let mailuser = env::var("BILLBOT_MAIL_USER")?;
    let mailpass = env::var("BILLBOT_MAIL_PASSWORD")?;
    let tls = native_tls::TlsConnector::builder().build()?;
    let client = imap::connect((config.imap_server.to_string(), config.imap_port), config.imap_server.to_string(), &tls)?;

    let imap_session = client
        .login(&mailuser, mailpass)
        .map_err(|e| e.0)?;

    Ok(imap_session)
}

pub fn create_mail_draft(mailconfig_global: &MailConfig, mailconfig_company: &CompanyConfig, pdf_content: Vec<u8>, pdf_name: &str) -> MailResult<()> {

    let mut imap_session = get_imap_session(mailconfig_global)?;

    let text = format!(
        "{greeting_to}\n\n{body}\n\n{greeting_from}\n",
        greeting_to = mailconfig_company.greeting_to,
        body = mailconfig_global.email_text,
        greeting_from = mailconfig_company.greeting_from,
    );

    let textmail = SinglePart::builder()
        .header(header::ContentType::TEXT_PLAIN)
        .body(String::from(text));

    let content_type = ContentType::parse("application/pdf")?;
    let attachment = Attachment::new(pdf_name.to_string()).body(pdf_content, content_type);

    let email = Message::builder()
        .from(mailconfig_global.email.parse().unwrap())
        .to(mailconfig_company.email.parse().unwrap())
        .subject(mailconfig_company.subject.to_string())
        .multipart(
            MultiPart::mixed()
                .singlepart(textmail)
                .singlepart(attachment)
        )?;

    let email_bytes = email.formatted();

    imap_session.append("Drafts", &email_bytes)?;

    imap_session.logout()?;

    Ok(())
}