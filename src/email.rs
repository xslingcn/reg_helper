use crate::config::CONFIG;
use crate::register;
use mailparse::*;
use native_tls::TlsConnector;
use chrono::{Utc, TimeZone, Duration};
use regex::Regex;

pub fn fetch_email() -> imap::error::Result<Option<String>> {
    let tls = TlsConnector::builder().build().unwrap();
    let client = imap::connect((CONFIG.email.imap_host.as_str(), CONFIG.email.imap_port), &CONFIG.email.imap_host, &tls).unwrap();
    println!("Client prepared...");

    let mut imap_session = client
    .login(&CONFIG.email.imap_username, &CONFIG.email.imap_password)
    .map_err(|e| e.0)
    .unwrap_or_else(|err| {
        eprintln!("Failed to login: {}", err);
        std::process::exit(1);
    });
    println!("Logged in...");
    imap_session.select("INBOX")?;

    let messages = imap_session.search("UNSEEN")?;
    if messages.is_empty() {
        println!("No unread messages.");
    } else {
        for msg_id in messages.iter() {
            println!("Loading message `{}`", &msg_id);
            let messages = imap_session.fetch(msg_id.to_string(), "(RFC822.HEADER BODY[TEXT])")?;
            let message = if let Some(m) = messages.iter().next() {
                m
            } else {
                return Ok(None);
            };
            
            let body = message.text().expect("Message did not have a body!");

            let mail = mailparse::parse_mail(body).unwrap();
            let date_header = mail.headers.get_first_value("Date").unwrap();
            let from_header = mail.headers.get_first_value("From").unwrap();
            let content = mail.subparts[0].get_body().unwrap();
            println!("Email parsed: \n Date: `{}`\nFrom: `{}`\nContent: `{}`", date_header, from_header, content);

            if from_header.contains("notify-noreply@uw.edu") {
                println!("Notify.UW email found!");
                let date = mailparse::dateparse(&date_header).unwrap();
                let now = Utc::now();
                if now.signed_duration_since(Utc.timestamp_opt(date, 0).unwrap()) < Duration::minutes(1) {
                    let re = Regex::new(r"SLN: (\d{5})").unwrap();
                    if let Some(caps) = re.captures(&content) {
                        let sln = caps.get(1).unwrap().as_str();
                        println!("Extracted SLN: {}", sln);

                        register::register(sln);

                        imap_session.store(msg_id.to_string(), "+FLAGS (\\Seen)")?;
                    }
                }
            }
        }
    }

    imap_session.logout()?;
    Ok(Some("Logged Out!".to_string()))
}