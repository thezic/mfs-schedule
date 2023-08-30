mod database;
mod export;

use serde::{Deserialize, Serialize};
use std::fs;

use database::Database;
use export::Export;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub database: Database,
    pub export: Export,
}

#[derive(Debug, Deserialize)]
struct ConfigInput {
    pub database: Option<Database>,
    pub export: Option<Export>,
}

impl Config {
    fn new(app: &tauri::App) -> Config {
        Config {
            database: Database::new(app),
            export: Export::default(),
        }
    }

    fn from_config_input(app: &tauri::App, input: ConfigInput) -> Config {
        Config {
            database: input.database.unwrap_or(Database::new(app)),
            export: input.export.unwrap_or(Export::default()),
        }
    }

    pub fn load(path: &str, app: &tauri::App) -> Config {
        let parsed_config: Option<ConfigInput> = match fs::read_to_string(path) {
            Ok(content) => toml::from_str(&content).expect("Unable to parse config-file"),
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => None, //Config::new(app),
                _ => panic!("Unable to open config file"),
            },
        };

        match parsed_config {
            Some(input) => Config::from_config_input(app, input),
            None => Config::new(app),
        }
    }
}
