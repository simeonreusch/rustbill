use imap;
use imap::types::Flag;
use native_tls;
use native_tls::TlsStream;
use chrono::{NaiveDate, Locale};
use std::net::TcpStream;
use std::env;
use lettre::message::{header::ContentType, header, Attachment, SinglePart, MultiPart, Message};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{SmtpTransport, Transport};
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
    #[error("SMTP error. Check credentials or connection.")]
    SMTPError(#[from] lettre::transport::smtp::Error),
    #[error("Content type error")]
    ContentTypeError(#[from] header::ContentTypeErr),
    #[error("TLS error")]
    TLSError(#[from] native_tls::Error)
}

type MailResult<T> = Result<T, MailError>;

fn get_credentials() -> MailResult<(String, String)> {
    dotenv().ok();
    let mailuser = env::var("RUSTBILL_MAIL_USER")?;
    let mailpass = env::var("RUSTBILL_MAIL_PASSWORD")?;
    Ok((mailuser, mailpass))
}

fn get_imap_session(config: &MailConfig) -> MailResult<imap::Session<TlsStream<TcpStream>>> {
    let (mailuser, mailpass) = get_credentials()?;
    let tls = native_tls::TlsConnector::builder().build()?;
    let client = imap::connect((config.imap_server.to_string(), config.imap_port), config.imap_server.to_string(), &tls)?;

    let imap_session = client
        .login(&mailuser, &mailpass)
        .map_err(|e| e.0)?;

    Ok(imap_session)
}

fn build_message(mailconfig_global: &MailConfig, mailconfig_company: &CompanyConfig, billdate: &NaiveDate, pdf_content: Vec<u8>, pdf_name: &str) -> MailResult<Message> {

    let mail_text = format!("{text} im {month} {year}.",
        text=mailconfig_global.email_text,
        month=billdate.format_localized("%B", Locale::de_DE),
        year=billdate.format("%Y"),
    );

    let text = format!(
        "{greeting_to}\n\n{body}\n\n{greeting_from}\n",
        greeting_to = mailconfig_company.greeting_to,
        body = mail_text,
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

    Ok(email)
}

pub fn create_mail_draft(mailconfig_global: &MailConfig, mailconfig_company: &CompanyConfig, billdate: &NaiveDate, pdf_content: Vec<u8>, pdf_name: &str) -> MailResult<()> {

    let mut imap_session = get_imap_session(mailconfig_global)?;

    let email = build_message(mailconfig_global, mailconfig_company, billdate, pdf_content, pdf_name)?;

    imap_session.append("Drafts", &email.formatted())?;

    imap_session.logout()?;

    Ok(())
}

pub fn send_mail(mailconfig_global: &MailConfig, mailconfig_company: &CompanyConfig, billdate: &NaiveDate, pdf_content: Vec<u8>, pdf_name: &str) -> MailResult<()> {

    let email = build_message(mailconfig_global, mailconfig_company, billdate, pdf_content, pdf_name)?;
    let email_bytes = email.formatted();

    // Send via SMTP. A relay transport uses implicit TLS (port 465).
    let credentials = get_credentials()?;
    let sender = SmtpTransport::relay(&mailconfig_global.smtp_server)?
        .port(mailconfig_global.smtp_port)
        .credentials(Credentials::new(credentials.0, credentials.1))
        .build();

    sender.send(&email)?;

    // SMTP does not store sent messages server-side, so append the
    // identical message to the sent folder via IMAP for tracking.
    // The \Seen flag marks it as read.
    let mut imap_session = get_imap_session(mailconfig_global)?;

    imap_session.append_with_flags(&mailconfig_global.sent_folder, &email_bytes, &[Flag::Seen])?;

    imap_session.logout()?;

    Ok(())
}
