// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
use commands::*;

use std::env;

use specta::collect_types;
use tauri::Manager;
use tauri::RunEvent;
use tauri_specta::ts;

use app::app::AppState;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    #[cfg(debug_assertions)]
    export_bindings();
    simple_logger::SimpleLogger::new().init().unwrap();

    let app = tauri::Builder::default()
        .setup(|app| {
            let app_state = async_std::task::block_on(AppState::init(app));
            app.manage(app_state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            create_person,
            get_persons,
            get_person_by_id,
            delete_person,
            update_person,
            get_planned_events,
            create_event,
            update_event,
            delete_event,
            export_pdf,
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    app.run(|app_handle, event| {
        if let RunEvent::Exit = event {
            let state: tauri::State<'_, AppState> = app_handle.state();
            println!("Exit, cleanup");
            async_std::task::block_on(state.cleanup());
        }
    });

    Ok(())
}

fn export_bindings() {
    ts::export(
        collect_types![
            create_person,
            delete_person,
            get_person_by_id,
            get_persons,
            update_person,
            get_planned_events,
            create_event,
            update_event,
            delete_event,
            export_pdf,
        ],
        "../src/bindings.ts",
    )
    .unwrap();
}
