// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri_plugin_log::{Target, TargetKind};
use tauri_plugin_log::fern::colors::{Color, ColoredLevelConfig};
use crate::commands::app_image_commands::{install_app, read_app_list, uninstall_app};

use crate::commands::app_settings_commands::{read_settings, save_settings};
use crate::commands::dialog_commands::{pick_app_icon, pick_app_image, pick_dir};
use crate::models::app_state::AppState;

mod commands;
mod helpers;
mod models;

fn main() {
    tauri::Builder::default()
        .setup(|app| Ok(()))
        .manage(AppState {
            state: std::sync::Mutex::new(None),
        })
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_persisted_scope::init())
        .plugin(tauri_plugin_single_instance::init(|app, args, cwd| {}))
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    Target::new(TargetKind::Stdout),
                ])
                .with_colors(
                    ColoredLevelConfig::new()
                        .info(Color::Green)
                        .warn(Color::Yellow)
                        .error(Color::Red)
                        .debug(Color::Cyan)
                        .trace(Color::White),
                )
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            pick_app_image,
            pick_app_icon,
            pick_dir,
            install_app,
            read_app_list,
            uninstall_app,
            read_settings,
            save_settings
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
