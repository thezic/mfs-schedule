use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Database {
    pub path: String,
}

impl Database {
    fn new(app: &tauri::App) -> Database {
        let tauri_config = app.config();
        let parts = tauri_config
            .tauri
            .bundle
            .identifier
            .split('.')
            .collect::<Vec<_>>()
            .to_vec();
        let (qualifier, organization, application) = (parts[0], parts[1], parts[2]);

        let project_dirs = ProjectDirs::from(qualifier, organization, application)
            .expect("Unable to get default directories");
        let data_path = project_dirs.data_local_dir();

        let db_path = data_path.join("database.sqlite");

        Database {
            path: db_path.display().to_string(),
        }
        // Database::default()
        //Database { path: ProjectDirs::from() }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub database: Database,
}

impl Config {
    fn new(app: &tauri::App) -> Config {
        Config {
            database: Database::new(app),
            ..Config::default()
        }
    }
}

// impl Default for Database {
//     fn default() -> Self {
//         Database {
//             path: ProjectDirs::
//         }
//     }
// }

impl Config {
    pub fn load(path: &str, app: &tauri::App) -> Config {
        match fs::read_to_string(path) {
            Ok(content) => toml::from_str(&content).expect("Unable to parse config-file"),
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => Config::new(app),
                _ => panic!("Unable to open config file"),
            },
        } // .expect("Unable to open config file");
          // toml::from_str(&config_string).unwrap()
    }
}
