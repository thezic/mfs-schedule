// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app::connect;
use std::env;

#[tauri::command]
fn create_person(name: &str) {
    let dir = env::current_dir().unwrap();
    println!("Current working directory: {}", dir.display());
    println!("Create person: {name}");
    let connection = &mut connect();

    app::create_person(connection, name);
}

#[tauri::command]
fn get_persons() -> Vec<String> {
    let connection = &mut connect();
    app::get_persons(connection)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![create_person, get_persons])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
