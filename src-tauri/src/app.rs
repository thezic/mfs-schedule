use sqlx::sqlite::SqlitePoolOptions;

use crate::{
    config::Config, core::services::Service,
    infrastructure::datastore::repository::person::PersonRepository,
};

pub struct App {
    pub config: Config,
    pub db_pool: sqlx::Pool<sqlx::Sqlite>,
}

impl App {
    pub async fn init() -> App {
        let config = Config::load("mfs.config.toml");

        let db_pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect(&config.database.path)
            .await
            .unwrap();

        App { config, db_pool }
    }

    pub async fn service(&self) -> Service {
        let repo = self.get_persons_repo().await;
        Service::new(Box::new(repo))
    }

    pub async fn get_persons_repo(&self) -> PersonRepository {
        let conn = self.db_pool.acquire().await.unwrap();
        PersonRepository::new(conn)
    }
}
