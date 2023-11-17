use chrono::Local;
use tokio::time::{self, Duration, Instant};
mod config;
mod email;
mod register;

#[tokio::main]
async fn main() {
    let mut interval = time::interval(Duration::from_secs(15));
    let mut session_reinit_timer = time::interval(Duration::from_secs(1 * 60 * 60));
    let mut shib_session_refresh_timer = time::interval(Duration::from_secs(1 * 60 * 60));

    // loop {
    //     tokio::select! {
    //         _ = shib_session_refresh_timer.tick() => {
    //             match register::refresh_shib_session().await {
    //                 Ok(r) => println!("{}", r),
    //                 Err(err) => {
    //                     eprintln!("Error refreshing shib session: {}", err);
    //                     return;
    //                 }
    //             }
    //         }
    //     }
    //     let mut imap_session = match email::init_imap_session() {
    //         Ok(s) => s,
    //         Err(e) => {
    //             eprintln!("Failed to initialize IMAP session: {}", e);
    //             return;
    //         }
    //     };
    //     match email::idle(&mut imap_session).await {
    //         Ok(r) => println!("{}", r),
    //         Err(err) => eprintln!("{}", err),
    //     }
    //     match email::close_imap_session(&mut imap_session) {
    //         Ok(_) => println!("Session closed."),
    //         Err(err) => eprintln!("Failed to close IMAP session: {}", err),
    //     }
    // }

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
            Err(err) => eprintln!("{}", err),
        }
    }
    _ = session_reinit_timer.tick() => {
        if session_init_time.elapsed() < Duration::from_secs(1 * 60 * 60) {
            continue;
        }
        match email::close_imap_session(&mut session) {
            Ok(_) => println!("Session closed."),
            Err(err) => eprintln!("Failed to close IMAP session: {}", err),
        }

        match email::init_imap_session() {
            Ok(new_session) => {
                println!("IMAP session reinitialized.");
                session = new_session;
            }
            Err(e) => {
                eprintln!("Failed to reinitialize IMAP session: {}", e);
                return;
            }
        }
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
