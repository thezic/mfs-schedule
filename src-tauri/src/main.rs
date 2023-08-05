// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use sqlx::sqlite::SqlitePoolOptions;
use std::env;

use app::config::Config;
// use app::connect;

struct App {
    pub config: Config,
    pub db_pool: sqlx::Pool<sqlx::Sqlite>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct MyError {
    error: String,
}

impl From<sqlx::Error> for MyError {
    fn from(value: sqlx::Error) -> Self {
        MyError {
            error: value.to_string(),
        }
    }
}

#[tauri::command]
async fn create_person(app: tauri::State<'_, App>, name: &str) -> Result<i64, MyError> {
    let dir = env::current_dir().unwrap();
    println!("Current working directory: {}", dir.display());
    println!("Create person: {name}");

    let mut conn = app.db_pool.acquire().await?;

    let id = sqlx::query!("INSERT INTO persons (name) VALUES ($1)", name)
        .execute(&mut *conn)
        .await?
        .last_insert_rowid();

    Ok(id)
}

#[tauri::command]
async fn get_persons(app: tauri::State<'_, App>) -> Result<Vec<String>, MyError> {
    println!("path {}", &app.config.database.path);

    let mut conn = app.db_pool.acquire().await?;

    let names = sqlx::query_scalar!("SELECT name FROM persons")
        .fetch_all(&mut *conn)
        .await?;

    Ok(names)
}

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::load("mfs.config.toml");

    let db_pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect(&config.database.path)
        .await?;

    let app = App { config, db_pool };

    tauri::Builder::default()
        .manage(app)
        .invoke_handler(tauri::generate_handler![create_person, get_persons])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
