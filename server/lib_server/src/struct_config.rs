use rocket::serde::Deserialize;
use std::fs::read_to_string;

#[derive(Deserialize)]
pub struct Config {
    pub log: String,
    pub address: String,
    pub port: u16,
    pub images: String,
    pub db: String,
    pub mqtt_host: String,
    pub mqtt_port: u16,
}

impl Config {
    pub fn load(path: &str) -> Config {
        let str_val: String = match read_to_string(path) {
            Ok(s) => s,
            Err(_) => panic!("[config] error reading {}", path),
        };
        match toml::from_str::<Config>(&str_val) {
            Ok(config) => config,
            Err(e) => panic!("[config] error parsing {} {}", path, e),
        }
    }
}
