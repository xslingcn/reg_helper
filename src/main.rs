mod config;
mod email;
mod register;

fn main() {
    println!("Logging into IMAP...");
    match email::init_imap_session() {
        Ok(mut session) => {
            match register::update_cookie() {
                Ok(r) => println!("{}", r),
                Err(err) => eprintln!("{}", err),
            }

            println!("Fetching Email...");
            match email::fetch_email(&mut session) {
                Ok(r) => println!("{}", r),
                Err(err) => eprintln!("{}", err),
            }

            match email::close_imap_session(&mut session) {
                Ok(r) => println!("{}", r),
                Err(err) => eprintln!("{}", err),
            }
        }
        Err(e) => eprintln!("Failed to initialize IMAP session: {}", e),
    }
}
