use std::io::{self, BufRead};

use chrono::Local;
use tokio::time::{self, Duration, Instant};

mod config;
mod email;
mod register;
mod webdrive;
mod cookie;
mod error;
mod switch;
mod command;

#[tokio::main]
async fn main() {
    let _ = color_eyre::install();

    tokio::spawn({
        async move {
            let stdin = io::stdin();
            let reader = io::BufReader::new(stdin);
            let mut lines = reader.lines();

            while let Some(Ok(line)) = lines.next() {
                println!("Received command: {}", line);
                match command::handle_command(line).await {
                    Ok(r) => println!("{}", r),
                    Err(err) => match err{
                        error::RegError::CommandNotFound(_) => {
                            eprintln!("Error parsing command: {}", err);
                        },
                        _ =>  eprintln!("{}", err),
                    }
                }
            }
        }
    });

    webdrive::saml_login().await.unwrap();

    let mut interval = time::interval(Duration::from_secs(15));
    let mut session_reinit_timer = time::interval(Duration::from_secs(1 * 60 * 60));
    let mut shib_session_refresh_timer = time::interval(Duration::from_secs(1 * 60 * 60));

    println!("Logging into IMAP...");
    let mut session_init_time = Instant::now();
    let mut session = match email::init_imap_session() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to initialize IMAP session: {}", e);
            return;
        }
    };

    loop {
        tokio::select! {
    _ = interval.tick() => {
        println!("Fetching Email at: {}", Local::now().format("%Y-%m-%d %H:%M:%S"));
        match email::fetch_email(&mut session).await {
            Ok(r) => println!("{}", r),
            Err(err) => match err{
                error::RegError::IMAPError(_) => {
                    eprintln!("Error fetching email: {}", err);
                    email::reinit_imap_session(&mut session).await;
                },
                _ =>  eprintln!("{}", err),
            }
        }
    }
    _ = session_reinit_timer.tick() => {
        if session_init_time.elapsed() < Duration::from_secs(1 * 60 * 60) {
            continue;
        }
        email::reinit_imap_session(&mut session).await;
        session_init_time = Instant::now();
    }
    _ = shib_session_refresh_timer.tick() => {
        match register::refresh_shib_session().await {
            Ok(r) => println!("{}", r),
            Err(err) => {
                eprintln!("Error refreshing shib session: {}", err);
                return;
            }
        }
    }
        }
    }
}