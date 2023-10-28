use chrono::NaiveDate;
use std::env;

use app::core::entities::ministry_event::MinistryEvent;
use app::core::entities::ministry_event::NewMinistryEvent;
use app::core::errors::DataStoreError;
use app::core::errors::ExportError;
use app::core::services::Context;

use app::app::AppState;
use app::core::entities::person::NewPerson;
use app::core::entities::*;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct MyError {
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

impl From<DataStoreError> for MyError {
    fn from(value: DataStoreError) -> Self {
        MyError {
            error: value.to_string(),
        }
    }
}

impl From<ExportError> for MyError {
    fn from(value: ExportError) -> Self {
        MyError {
            error: value.to_string(),
        }
    }
}

#[tauri::command]
#[specta::specta]
pub async fn create_person(
    app: tauri::State<'_, AppState>,
    new_person: NewPerson<'_>,
) -> Result<person::Person, MyError> {
    let dir = env::current_dir().unwrap();
    println!("Current working directory: {}", dir.display());
    println!("Create person: {:#?}", new_person);

    Ok(app.schedule_service().create_person(&new_person).await?)
}

#[tauri::command]
#[specta::specta]
pub async fn get_persons(app: tauri::State<'_, AppState>) -> Result<Vec<person::Person>, MyError> {
    Ok(app.schedule_service().get_persons().await?)
}

#[tauri::command]
#[specta::specta]
pub async fn delete_person(app: tauri::State<'_, AppState>, id: i32) -> Result<(), MyError> {
    app.schedule_service().delete_person(id.into()).await?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn get_person_by_id(
    app: tauri::State<'_, AppState>,
    id: i32,
) -> Result<person::Person, MyError> {
    Ok(app.schedule_service().get_person_by_id(id.into()).await?)
}

#[tauri::command]
#[specta::specta]
pub async fn update_person(
    app: tauri::State<'_, AppState>,
    person: person::Person,
) -> Result<person::Person, MyError> {
    Ok(app.schedule_service().update_person(person).await?)
}

#[tauri::command]
#[specta::specta]
pub async fn get_planned_events(
    app: tauri::State<'_, AppState>,
) -> Result<Vec<MinistryEvent>, MyError> {
    Ok(app.schedule_service().get_planned_events().await?)
}

#[tauri::command]
#[specta::specta]
pub async fn create_event(
    app: tauri::State<'_, AppState>,
    new_event: NewMinistryEvent,
) -> Result<MinistryEvent, MyError> {
    let event = app.schedule_service().create_event(&new_event).await?;
    Ok(event)
}

#[tauri::command]
#[specta::specta]
pub async fn update_event(
    app: tauri::State<'_, AppState>,
    event: MinistryEvent,
) -> Result<MinistryEvent, MyError> {
    Ok(app.schedule_service().update_event(event).await?)
}

#[tauri::command]
#[specta::specta]
pub async fn delete_event(app: tauri::State<'_, AppState>, id: i32) -> Result<(), MyError> {
    Ok(app.schedule_service().delete_event(id.into()).await?)
}

#[tauri::command]
#[specta::specta]
pub async fn export_pdf(
    app: tauri::State<'_, AppState>,
    handle: tauri::AppHandle,
    from: NaiveDate,
    to: NaiveDate,
    context: Context,
) -> Result<std::path::PathBuf, MyError> {
    Ok(app
        .export_service(handle)
        .export_pdf(from, to, context)
        .await?)
}
