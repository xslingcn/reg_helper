use crate::config::CONFIG;
use crate::register;
use chrono::{Duration, TimeZone, Utc};
use mailparse::*;
use native_tls::TlsConnector;
use regex::Regex;

pub fn fetch_email() -> Result<String, imap::Error> {
    let tls = TlsConnector::builder().build().unwrap();
    let client = imap::connect(
        (CONFIG.email.imap_host.as_str(), CONFIG.email.imap_port),
        &CONFIG.email.imap_host,
        &tls,
    )
    .unwrap();
    println!("Client prepared...");

    let mut imap_session = client
        .login(&CONFIG.email.imap_username, &CONFIG.email.imap_password)
        .map_err(|e| e.0)?;
    println!("Logged in...");

    imap_session.select("INBOX")?;
    println!("Inbox selected...");

    let yesterday = (Utc::now() - Duration::days(1)).date_naive();
    let date_str = yesterday.format("%d-%b-%Y").to_string();
    let query = format!("UNSEEN SINCE {}", date_str);
    let messages = imap_session.search(query)?;
    println!("Messages found: {:?}", messages);

    if messages.is_empty() {
        println!("No unread messages.");
    } else {
        for msg_id in messages.iter() {
            println!("Loading message `{}`", &msg_id);
            let message_chain = imap_session.fetch(msg_id.to_string(), "RFC822")?;
            let message = if let Some(m) = message_chain.iter().next() {
                m
            } else {
                eprintln!("Failed to find message with ID `{}`", msg_id);
                continue;
            };

            let body = message.body().expect("Message did not have a body!");

            let mail = mailparse::parse_mail(body).unwrap();
            let (date_header, from_header, content) = match (
                mail.headers.get_first_value("Date"),
                mail.headers.get_first_value("From"),
                mail.subparts.get(0).and_then(|p| p.get_body().ok()),
            ) {
                (Some(date), Some(from), Some(content)) => (date, from, content),
                _ => continue,
            };
            println!(
                "Email parsed: \nDate: `{}`\nFrom: `{}`\nContent: `{}`",
                date_header, from_header, content
            );

            if from_header.contains("me@xsl.sh") {
                println!("Notify.UW email found!");
                let date = mailparse::dateparse(&date_header).unwrap();
                let now = Utc::now();
                if now.signed_duration_since(Utc.timestamp_opt(date, 0).unwrap())
                    < Duration::minutes(1)
                {
                    let re = Regex::new(r"SLN: (\d{5})").unwrap();
                    if let Some(caps) = re.captures(&content) {
                        let sln = caps.get(1).unwrap().as_str();
                        println!("Extracted SLN: {}", &sln);

                        register::register(sln).map_err(|err| {
                            eprintln!("Failed to register: {}", err);
                            imap::Error::Bad("Failed to register".to_string())
                        })?;

                        imap_session.store(msg_id.to_string(), "+FLAGS (\\Seen)")?;
                    }
                } else {
                    println!("That's been too long ago (> 1 min)");
                }
            }
        }
    }

    imap_session.logout()?;
    Ok("Logged Out!".to_string())
}
