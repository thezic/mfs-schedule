use async_mutex::Mutex;
use sqlx::sqlite::SqlitePoolOptions;
use std::sync::Arc;

use crate::{
    config::Config, core::services::Service,
    infrastructure::datastore::repository::ministry_event::MinistryEventRepository,
    infrastructure::datastore::repository::person::PersonRepository,
};

pub struct AppState {
    pub config: Config,
    pub db_pool: Arc<Mutex<sqlx::Pool<sqlx::Sqlite>>>,
}

impl AppState {
    pub async fn init(app: &tauri::App) -> AppState {
        let config = Config::load("mfs.config.toml", app);

        println!("Using config {:#?}", config);

        let path = std::path::Path::new(&config.database.path);

        if !path.exists() {
            println!("Creating empty database at: {}", &config.database.path);
            if let Some(db_folder) = path.parent() {
                std::fs::create_dir_all(db_folder).unwrap_or_else(|err| {
                    panic!(
                        "Unable to create data folder at {}. ({})",
                        &db_folder.display(),
                        err
                    )
                });
            }
            std::fs::File::create(&config.database.path).expect("Unable to create database");
        }

        let db_pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect(&config.database.path)
            .await
            .unwrap();

        println!("pool ready, running migrations (if needed)");
        sqlx::migrate!()
            .run(&db_pool)
            .await
            .unwrap_or_else(|err| panic!("Unable to migrate data: {err}"));

        let db_pool = Arc::new(Mutex::new(db_pool));

        AppState { config, db_pool }
    }

    pub fn service(&self) -> Service {
        let persons_repo = PersonRepository::new(self.db_pool.clone());
        let ministry_event_repo = MinistryEventRepository::new(self.db_pool.clone());

        Service::new(Box::new(persons_repo), Box::new(ministry_event_repo))
    }

    pub async fn cleanup(&self) {
        let pool = self.db_pool.lock().await;
        (*pool).close().await
    }
}
