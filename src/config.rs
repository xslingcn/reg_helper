use lazy_static::lazy_static;
use serde_derive::{Serialize, Deserialize};
use std::fs;
use std::process::exit;
use std::sync::{Mutex, MutexGuard};
use toml;

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub email: Email,
    pub reg: Registration,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Email {
    pub imap_host: String,
    pub imap_username: String,
    pub imap_password: String,
    pub imap_port: u16,
    pub imap_ssl: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Registration {
    pub cookie: String,
    pub cw: String,
    pub quarter: u8,
    pub year: u16,
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

    pub fn update_cookie(&mut self, new_cookie: String) {
        self.reg.cookie = new_cookie;
    }

    pub fn save_to_file(&self, path: &str) {
        let toml = toml::to_string(self).expect("Error serializing config");
        fs::write(path, toml).expect("Error writing to file");
    }
}

lazy_static! {
    static ref CONFIG: Mutex<Config> = Mutex::new(Config::new("config.toml"));
}

pub fn update_global_cookie(new_cookie: String) {
    let mut config = CONFIG.lock().unwrap();
    config.update_cookie(new_cookie.clone());
    config.save_to_file("config.toml");
    *config = Config::new("config.toml");
}

pub fn get_config() -> MutexGuard<'static, Config> {
    CONFIG.lock().unwrap()
}