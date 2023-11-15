mod email;
mod register;
mod config;

fn main() {
    println!("Fetching Email...");
    email::fetch_email();
}
