use sqlx::sqlite::SqlitePoolOptions;

use crate::{
    config::Config, core::services::Service,
    infrastructure::datastore::repository::person::PersonRepository,
};

pub struct AppState {
    pub config: Config,
    pub db_pool: sqlx::Pool<sqlx::Sqlite>,
}

impl AppState {
    pub async fn init() -> AppState {
        let config = Config::load("mfs.config.toml");

        let db_pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect(&config.database.path)
            .await
            .unwrap();

        AppState { config, db_pool }
    }

    pub async fn service(&self) -> Service {
        let repo = self.get_persons_repo().await;
        Service::new(Box::new(repo))
    }

    pub async fn get_persons_repo(&self) -> PersonRepository {
        let conn = self.db_pool.acquire().await.unwrap();
        PersonRepository::new(conn)
    }

    pub async fn cleanup(&self) {
        self.db_pool.close().await
    }
}

// impl std::ops::Drop for AppState {
//     fn drop(&mut self) {
//         println!("Closing pool");
//         async_std::task::block_on(self.db_pool.close());
//     }
// }
