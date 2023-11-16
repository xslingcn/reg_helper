use std::{thread, time};
use chrono::Local;

mod config;
mod email;
mod register;

fn main() {
    println!("Logging into IMAP...");
    match email::init_imap_session() {
        Ok(mut session) => {
            loop {
                println!("Fetching Email at: {}", Local::now().format("%Y-%m-%d %H:%M:%S"));
                match email::fetch_email(&mut session) {
                    Ok(r) => println!("{}", r),
                    Err(err) => eprintln!("{}", err),
                }
                thread::sleep(time::Duration::from_secs(30));
            }
            // match email::close_imap_session(&mut session) {
            //     Ok(r) => println!("{}", r),
            //     Err(err) => eprintln!("{}", err),
            // }
        }
        Err(e) => eprintln!("Failed to initialize IMAP session: {}", e),
    }
}
