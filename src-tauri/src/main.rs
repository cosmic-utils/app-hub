// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri_plugin_store::StoreBuilder;
use crate::commands::app_settings_commands::{read_settings, save_settings};
use crate::models::app_state::AppState;

mod commands;
mod models;
mod helpers;

fn main() {
    tauri::Builder::default()
        .setup(|app| {Ok(())})
        .manage(AppState {
            state: std::sync::Mutex::new(None),
        })
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            commands::dialog_commands::pick_app_image,
            commands::dialog_commands::pick_app_icon,
            commands::dialog_commands::pick_dir,
            commands::app_image_commands::install_app,
            commands::app_image_commands::read_app_list,
            read_settings,
            save_settings
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
