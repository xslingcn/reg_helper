use chrono::{Duration, Local, TimeZone, Utc};
use imap::Session;
use mail_parser::MessageParser;
use native_tls::{TlsConnector, TlsStream};
use regex::Regex;
use std::net::TcpStream;

use crate::register;
use crate::{config::CONFIG, error::RegResult};

pub(crate) fn init_imap_session() -> RegResult<Session<TlsStream<TcpStream>>> {
    let tls = TlsConnector::builder().build()?;
    let client = imap::connect(
        (CONFIG.email.imap_host.as_str(), CONFIG.email.imap_port),
        &CONFIG.email.imap_host,
        &tls,
    )?;
    println!("Client prepared...");

    let imap_session = client
        .login(&CONFIG.email.imap_username, &CONFIG.email.imap_password)
        .map_err(|e| e.0)?;

    println!("Logged in...");
    Ok(imap_session)
}

pub(crate) fn close_imap_session(imap_session: &mut Session<TlsStream<TcpStream>>) -> RegResult<String> {
    imap_session.logout()?;
    Ok("Logged out...".to_string())
}

pub(crate) async fn idle(imap_session: &mut Session<TlsStream<TcpStream>>) -> RegResult<String> {
    imap_session.select("INBOX")?;

    let idle = imap_session.idle()?;
    println!("Idling...");

    idle.wait_with_timeout(std::time::Duration::from_secs(15 * 60))?;
    println!(
        "Update received on {}",
        Local::now().format("%Y-%m-%d %H:%M:%S")
    );
    return fetch_email(imap_session).await;
}

pub(crate) async fn fetch_email(imap_session: &mut Session<TlsStream<TcpStream>>) -> RegResult<String> {
    imap_session.select("INBOX")?;

    let yesterday = (Utc::now() - Duration::try_days(1).unwrap()).date_naive();
    let date_str = yesterday.format("%d-%b-%Y").to_string();
    let query = format!("UNSEEN SINCE {}", date_str);
    let messages = imap_session.search(query)?;

    if messages.is_empty() {
        return Ok("No unread messages.".to_string());
    } else {
        for msg_id in messages.iter() {
            // println!("Loading message `{}`", &msg_id);
            let message_chain = imap_session.fetch(msg_id.to_string(), "RFC822")?;
            let message = if let Some(m) = message_chain.iter().next() {
                m
            } else {
                eprintln!("Failed to find message with ID `{}`", msg_id);
                continue;
            };

            let body = message.body().expect("Message did not have a body!");
            let text = std::str::from_utf8(body)
                .expect("message was not valid utf-8")
                .to_string();

            let mail = MessageParser::default().parse(&text).unwrap();
            let (date, from, content) =
                match (mail.date(), mail.from().unwrap().first(), mail.body_text(0)) {
                    (Some(date), Some(from), Some(content)) => (date, from, content),
                    _ => {
                        println!("Failed to parse email with ID `{}`", msg_id);
                        continue;
                    }
                };

            // notify-noreply@uw.edu
            if from.address().unwrap().contains("notify-noreply@uw.edu") {                
                println!("Notify.UW email found!");
                println!(
                    "Date: `{}`\nFrom: `{}`\nContent: `{}`",
                    date.to_timezone(Local::now().offset().local_minus_utc() as i64).to_rfc3339(),
                    from.address().unwrap(),
                    content
                );
                imap_session.store(msg_id.to_string(), "+FLAGS (\\Seen)")?;
                let now = Utc::now();
                if now.signed_duration_since(Utc.timestamp_opt(date.to_timestamp(), 0).unwrap())
                    < Duration::try_minutes(3).unwrap()
                {
                    let re = Regex::new(r"SLN: (\d{5})").unwrap();
                    if let Some(caps) = re.captures(&content) {
                        let sln = caps.get(1).unwrap().as_str();
                        println!("Extracted SLN: {}", &sln);
                        return register::register(sln).await;
                    }
                } else {
                    println!("That's been too long ago (> 3 min) :(");
                }
            } else {
                println!("Not from Notify.UW.");
            }
        }
        Ok("Read complete and nothing interesting.".to_string())
    }
}

pub(crate) async fn reinit_imap_session(session: &mut Session<TlsStream<TcpStream>>) {
    match close_imap_session(session) {
        Ok(_) => println!("Session closed."),
        Err(err) => eprintln!("Failed to close IMAP session: {}", err),
    }

    match init_imap_session() {
        Ok(new_session) => {
            println!("IMAP session reinitialized.");
            *session = new_session;
        }
        Err(e) => {
            eprintln!("Failed to reinitialize IMAP session: {}", e);
            return;
        }
    }
}