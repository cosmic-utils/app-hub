// Documentation available at:
// https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/store#usage-from-rust
// plugin is in beta, so the API may change

use tauri::{AppHandle, Manager};

use crate::models::app_settings::AppSettings;
use crate::services::settings_service::{read_settings, save_settings};

#[tauri::command]
pub async fn read_settings_command(app: AppHandle) -> Result<AppSettings, String> {
   match read_settings(app) {
       Ok(settings) => {
          Ok(settings)
       }
       Err(error) => {
            Err(error)
       }
   }
}

#[tauri::command]
pub async fn save_settings_command(app: AppHandle, settings: AppSettings) -> Result<(), String> {
    match save_settings(app, settings).await {
        Ok(_) => {
            Ok(())
        }
        Err(error) => {
            Err(error)
        }
    }
}
