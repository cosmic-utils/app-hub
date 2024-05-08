use std::process::Command;
use log::{error, info};
use tauri::AppHandle;

use crate::models::app_list::{App, AppList};
use crate::models::request_installation::RequestInstallation;
use crate::services::app_image_service::{install_app_image, read_all_app, uninstall_app};

#[tauri::command]
pub async fn install_app_command(app: AppHandle, request_installation: RequestInstallation) -> Result<String, String> {
    match install_app_image(app, request_installation).await {
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
    match uninstall_app(app.name) {
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
