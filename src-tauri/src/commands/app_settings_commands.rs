
// Documentation available at:
// https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/store#usage-from-rust
// plugin is in beta, so the API may change

use std::path::PathBuf;

use tauri::{AppHandle, Manager, Wry};
use tauri_plugin_store::{StoreCollection, with_store};

use crate::models::app_settings::AppSettings;

#[tauri::command]
pub async fn read_settings(app: AppHandle) -> Result<AppSettings, String> {

    // Clone the app handle to avoid borrowing issues
    let app_clone = app.clone();

    // Get the store collection from the app state
    let stores = app.state::<StoreCollection<Wry>>();
    // Define the path to the settings file
    let path = PathBuf::from("app_settings.bin");

    // Use the cloned app handle to get the settings from the store
    let res = with_store(app_clone.clone(), stores.clone(), path.clone(), |store| {
        // Try to get the settings from the store
        match store.get("app_settings") {
            // If the settings are found, clone them and return
            Some(value) => Ok(Some(value.clone())),
            // If the settings are not found, return None
            None => Ok(None),
        }
    }).map_err(|e| e.to_string())?;

    // If the settings were not found, create and save default settings
    let res = match res {
        Some(res) => res,
        None => {
            let default_settings = AppSettings {
                theme: "dark".to_string(),
                language: "en".to_string(),
                install_path: Some(format!("{}/AppImages/", dirs::home_dir().unwrap().to_string_lossy())),
                create_desktop_entry: true,
            };
            let serialized_settings = serde_json::to_value(default_settings.clone()).map_err(|e| e.to_string())?;
            with_store(app_clone, stores, path, |store| store.insert("app_settings".to_string(), serialized_settings)).expect("error saving default settings");
            serde_json::to_value(default_settings).map_err(|e| e.to_string())?
        }
    };

    // Try to deserialize the settings
    let deserialized = serde_json::from_value::<AppSettings>(res).map_err(|e| e.to_string())?;

    // Print the deserialized settings for debugging
    println!("{:?}", deserialized);

    // Return the deserialized settings
    Ok(deserialized)
}

#[tauri::command]
pub async fn save_settings(app: AppHandle, settings: AppSettings) -> Result<(), String> {

    // Check if the install path is empty
    if let Some(install_path) = &settings.install_path {
        if install_path.is_empty() {
            return Err("Install path cannot be empty".into());
        }
    }

    // Check if the theme and language are empty
    if settings.theme.is_empty() || settings.language.is_empty() {
        return Err("Theme and language cannot be empty".into());
    }


    // Get the store collection from the app state
    let stores = app.state::<StoreCollection<Wry>>();
    // Define the path to the settings file
    let path = PathBuf::from("app_settings.bin");

    // Serialize the settings to JSON
    let serialized_settings = serde_json::to_value(settings).map_err(|e| e.to_string())?;

    // Use the cloned app handle to save the settings to the store
    let res = with_store(app.clone(), stores, path, |store| store.insert("app_settings".to_string(), serialized_settings));

    // Check the result of the save operation
    match res {
        // If the save was successful, print a success message
        Ok(_) => {
            println!("Settings saved successfully");
        }
        // If the save failed, print the error and return it
        Err(err) => {
            println!("{:?}", err);
            return Err("Error saving settings".to_string());
        }
    }

    // Return Ok if the function executed successfully
    Ok(())
}
