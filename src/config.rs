use lazy_static::lazy_static;
use serde_derive::Deserialize;
use std::fs;
use std::process::exit;
use toml;

use crate::error::{RegError, RegResult};

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
    pub fn new(path: &str) -> RegResult<Self> {
        let contents = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => Err(RegError::ConfigLoadError(format!("Could not read file `{}`", path)))?
        };

        let data: Config = match toml::from_str(&contents) {
            Ok(d) => d,
            Err(e) => Err(RegError::ConfigLoadError(format!("Unable to read config: `{}`", e)))?
        };

        println!("Config loaded!");
        Ok(data)
    }
}

lazy_static! {
    pub(crate) static ref CONFIG: Config = match Config::new("config.toml") {
        Ok(c) => c,
        Err(e) => {
            println!("{}", e);
            exit(1);
        }
    };
}
