use log::{error, info};
use tauri::AppHandle;
use crate::models::app_database::AppDatabase;
use crate::services::app_store_service::{get_app_list, install_app_from_remote, update_app_database};

#[tauri::command]
pub async fn update_database_command(app: AppHandle) -> Result<(), String> {
    match update_app_database(app).await {
        Ok(res) => {
            info!("Database updated");
            Ok(())
        }
        Err(err) => {
            error!("Failed to update database: {}", err);
            Err(err)
        }
    }
}

#[tauri::command]
pub async fn get_app_list_command(app: AppHandle) -> Result<AppDatabase, String> {
    match get_app_list(app).await {
        Ok(res) => {
            info!("App list fetched");
            Ok(res)
        }
        Err(err) => {
            error!("Failed to fetch app list: {}", err);
            Err(err)
        }
    }
}

#[tauri::command]
pub async fn install_app_from_remote_command(app: AppHandle, download_url: String, app_name: String) -> Result<(), String> {
    match install_app_from_remote(app, download_url, app_name).await {
        Ok(res) => {
            info!("App installed");
            Ok(())
        }
        Err(err) => {
            error!("Failed to install app: {}", err);
            Err(err)
        }
    }
}