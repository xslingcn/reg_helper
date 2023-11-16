mod email;
mod register;
mod config;

fn main() {
    println!("Fetching Email...");
    match email::fetch_email() {
        Ok(r) => println!("{}", r),
        Err(err) => eprintln!("{}", err),
    }
}
