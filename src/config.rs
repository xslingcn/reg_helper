use lazy_static::lazy_static;
use serde_derive::Deserialize;
use std::fs;
use std::process::exit;
use toml;

#[derive(Deserialize)]
pub struct Config {
    pub email: Email,
    pub reg: Registration,
}

#[derive(Deserialize)]
pub struct Email {
    pub imap_host: String,
    pub imap_username: String,
    pub imap_password: String,
    pub imap_port: u16,
    pub imap_ssl: bool,
}

#[derive(Deserialize)]
pub struct Registration {
    pub cw: String,
    pub quarter: u8,
    pub year: u16,
    pub netid: String,
    pub password: String,
}

impl Config {
    pub fn new(path: &str) -> Self {
        let contents = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => {
                eprintln!("Could not read file `{}`", path);
                exit(1);
            }
        };

        let data: Config = match toml::from_str(&contents) {
            Ok(d) => d,
            Err(e) => {
                eprintln!("Unable to load data from `{}`", path);
                eprint!("Exception: `{}`", e);
                exit(1);
            }
        };

        println!("Config loaded!");
        data
    }
}

lazy_static! {
    pub static ref CONFIG: Config = Config::new("config.toml");
}
