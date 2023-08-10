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
    pub async fn init() -> AppState {
        let config = Config::load("mfs.config.toml");

        let db_pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect(&config.database.path)
            .await
            .unwrap();

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
