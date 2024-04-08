use dirs;
use log::{error, info, warn};
use tauri::AppHandle;

use crate::commands::app_settings_commands::read_settings_command;
use crate::helpers::app_images_helpers::{app_image_extract_all, app_image_extract_desktop_file, find_icons_paths, install_icons, update_icon_cache};
use crate::helpers::desktop_file_builder::DesktopFileBuilder;
use crate::helpers::desktop_file_helpers::{delete_desktop_file_by_name, find_desktop_entry, find_desktop_file_location};
use crate::helpers::file_system_helper::{add_executable_permission, find_desktop_file_in_dir, rm_dir_all, rm_file, sudo_remove_file};
use crate::models::app_list::{App, AppList};
use crate::models::request_installation::RequestInstallation;
use crate::services::app_image_service::{install_app_image, read_all_app, uninstall_app};

#[tauri::command]
pub async fn install_app_command(app: AppHandle, request_installation: RequestInstallation) -> Result<String, String> {
    match install_app_image(app, request_installation) {
        Ok(_) => {
            info!("App installed successfully");
            Ok("App installed successfully".to_string())
        }
        Err(err) => {
            error!("{}", err);
            Err(err)
        }
    }
}

#[tauri::command]
pub async fn read_app_list_command() -> Result<AppList, String> {
    let apps: Vec<App> = read_all_app()?;
    Ok(AppList { apps })
}

#[tauri::command]
pub async fn uninstall_app_command(app: App) -> Result<bool, String> {
    match uninstall_app(app) {
        Ok(uninstalled) => {
            info!("App uninstalled successfully");
            Ok(uninstalled)
        }
        Err(error) => {
            error!("{}", error);
            Err(error)
        }
    }
}
