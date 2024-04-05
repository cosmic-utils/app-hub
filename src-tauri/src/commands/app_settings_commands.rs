// Documentation available at:
// https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/store#usage-from-rust
// plugin is in beta, so the API may change

use std::fs;
use std::path::PathBuf;
use log::{debug, error, info, warn};

use tauri::{AppHandle, Manager, Wry};
use tauri_plugin_store::{with_store, StoreCollection};
use crate::helpers::desktop_file_builder::DesktopFileBuilder;
use crate::helpers::desktop_file_helpers::{find_desktop_entries_by_exec_contains};
use crate::helpers::file_system_helper::copy_dir_all;

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
    })
    .map_err(|e| e.to_string())?;

    // If the settings were not found, create and save default settings
    let res = match res {
        Some(res) => res,
        None => {
            let default_settings = AppSettings {
                theme: "cupcake".to_string(),
                language: "en".to_string(),
                install_path: Some(format!(
                    "{}/AppImages/",
                    dirs::home_dir().unwrap().to_string_lossy()
                )),
                create_desktop_entry: true,
            };
            let serialized_settings =
                serde_json::to_value(default_settings.clone()).map_err(|e| e.to_string())?;
            with_store(app_clone, stores, path, |store| {
                store.insert("app_settings".to_string(), serialized_settings)
            })
            .expect("error saving default settings");
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
        else {
            // Check if the install path is valid
            let path = PathBuf::from(install_path);
            if !path.exists() {
                return Err("Install path does not exist".into());
            }
            // Check if the new path is different from the old path
            let old_settings = read_settings(app.clone()).await?;
            if let Some(old_install_path) = old_settings.install_path {
                if !old_install_path.eq(install_path) {
                    info!("Install path changed from {} to {}", old_install_path, install_path);
                    // Move all AppImages and icons to the new path
                    let moved = copy_dir_all(
                        &old_install_path,
                        install_path
                    );
                    if let Err(e) = moved {
                        error!("Error moving AppImages and icons: {:?}", e);
                    }
                    info!("AppImages and icons moved successfully");

                    // Update the path in the .desktop files
                    let desktop_entries = find_desktop_entries_by_exec_contains(&old_install_path);

                    for desktop_entry_path in desktop_entries.unwrap() {
                        
                        match DesktopFileBuilder::from_desktop_entry_path(desktop_entry_path.clone()) {
                            Ok(mut desktop_file_builder) => {

                                // Check exec field
                                if desktop_file_builder.exec().is_none() {
                                    error!("Failed to parse desktop entry (exec missing): {}", desktop_entry_path);
                                    continue;
                                }

                                let new_exec = desktop_file_builder.exec().unwrap().replace(old_install_path.as_str(), install_path.as_str());
                                debug!("new exec: {}", new_exec);
                                desktop_file_builder.set_exec(new_exec);
                                if desktop_file_builder.path().is_some() {
                                    desktop_file_builder.set_path(desktop_file_builder.path().unwrap().replace(old_install_path.as_str(), install_path.as_str()));
                                }
                                if desktop_file_builder.icon().is_some() {
                                    desktop_file_builder.set_icon(desktop_file_builder.icon().unwrap().replace(old_install_path.as_str(), install_path.as_str()));
                                }
                                desktop_file_builder.write_to_file(desktop_entry_path).expect("Error updating .desktop file");
                            }
                            Err(error) => {
                                warn!("Error updating .desktop file: {}", error);
                            }
                        }
                    }
                }
            }
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
    let res = with_store(app.clone(), stores, path, |store| {
        store.insert("app_settings".to_string(), serialized_settings)
    });

    // Check the result of the save operation
    match res {
        Ok(_) => {
            info!("Settings saved successfully");
        }
        Err(err) => {
            info!("{:?}", err);
            return Err("Error saving settings".to_string());
        }
    }

    // Return Ok if the function executed successfully
    Ok(())
}
