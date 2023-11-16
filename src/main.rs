mod config;
mod email;
mod register;

fn main() {
    println!("Fetching Email...");
    match email::fetch_email() {
        Ok(r) => println!("{}", r),
        Err(err) => eprintln!("{}", err),
    }
}
