use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
    pub path: String,
}

impl Database {
    pub fn new(app: &tauri::App) -> Database {
        let tauri_config = app.config();
        let parts = tauri_config
            .tauri
            .bundle
            .identifier
            .split('.')
            .collect::<Vec<_>>();
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
