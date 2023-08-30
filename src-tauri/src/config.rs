mod database;
mod export;

use serde::{Deserialize, Serialize};
use std::fs;

use database::Database;
use export::Export;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub database: Database,
    pub export: Option<Export>,
}

impl Config {
    fn new(app: &tauri::App) -> Config {
        Config {
            database: Database::new(app),
            ..Config::default()
        }
    }
}

impl Config {
    pub fn load(path: &str, app: &tauri::App) -> Config {
        match fs::read_to_string(path) {
            Ok(content) => toml::from_str(&content).expect("Unable to parse config-file"),
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => Config::new(app),
                _ => panic!("Unable to open config file"),
            },
        }
    }
}
