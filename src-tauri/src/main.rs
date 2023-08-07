// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;

use app::app::App;
use app::core::entities::person::NewPerson;
use app::core::entities::*;
use app::core::traits::PersonRepository as PersonRepositoryTrait;

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

impl From<anyhow::Error> for MyError {
    fn from(value: anyhow::Error) -> Self {
        MyError {
            error: value.to_string(),
        }
    }
}

#[tauri::command]
async fn create_person(
    app: tauri::State<'_, App>,
    new_person: NewPerson<'_>,
) -> Result<person::Person, MyError> {
    let dir = env::current_dir().unwrap();
    println!("Current working directory: {}", dir.display());
    println!("Create person: {:#?}", new_person);

    Ok(app.service().await.create_person(&new_person).await)
}

#[tauri::command]
async fn get_persons(app: tauri::State<'_, App>) -> Result<Vec<person::Person>, MyError> {
    println!("path {}", &app.config.database.path);

    Ok(app.service().await.get_persons().await)
}

#[tauri::command]
async fn delete_person(app: tauri::State<'_, App>, id: i64) -> Result<(), MyError> {
    app.service().await.delete_person(id).await;
    Ok(())
}

#[tauri::command]
async fn get_person_by_id(app: tauri::State<'_, App>, id: i64) -> Result<person::Person, MyError> {
    Ok(app.service().await.get_person_by_id(id).await)
}

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let app = App::init().await;
    tauri::Builder::default()
        .manage(app)
        .invoke_handler(tauri::generate_handler![
            create_person,
            get_persons,
            get_person_by_id,
            delete_person,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
