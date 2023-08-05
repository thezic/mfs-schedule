use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Default, Serialize, Deserialize)]
pub struct Database {
    pub path: String,
}

#[derive(Default, Serialize, Deserialize)]
pub struct Config {
    pub database: Database,
}

impl Config {
    pub fn load(path: &str) -> Config {
        let config_string = fs::read_to_string(path).expect("Unable to open config file");
        toml::from_str(&config_string).unwrap()
    }
}
