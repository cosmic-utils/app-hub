// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::models::app_state::AppState;

mod commands;
mod models;

fn main() {
    tauri::Builder::default()
        .setup(|app| {Ok(())})
        .manage(AppState {
            state: std::sync::Mutex::new(None),
        })
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            commands::dialog_commands::choose_file,
            commands::app_image_commands::install_file,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
