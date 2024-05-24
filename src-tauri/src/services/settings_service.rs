use std::fs;
use std::fs::{File, OpenOptions, remove_dir_all};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use log::{debug, error, info, warn};
use tauri::{AppHandle, Manager, Wry};
use tauri_plugin_store::{StoreCollection, with_store};
use common_utils::desktop_file_builder::DesktopFileBuilder;
use common_utils::desktop_file_helpers::find_desktop_entries_by_exec_contains;
use common_utils::file_system_helpers::copy_dir_all;
use crate::commands::app_settings_commands::read_settings_command;
use crate::models::app_settings::AppSettings;

/// Read the app settings from the store
pub fn read_settings(app: AppHandle) -> Result<AppSettings, String> {
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
                theme: "cupcake".to_string(),
                language: "en".to_string(),
                install_path: Some(format!(
                    "{}/AppImages/",
                    dirs::home_dir().unwrap().to_string_lossy()
                )),
            };
            let serialized_settings = serde_json::to_value(default_settings.clone()).map_err(|e| e.to_string())?;
            with_store(app_clone, stores, path, |store| {
                store.insert("app_settings".to_string(), serialized_settings.clone())
            }).expect("error saving default settings");
            // create the default path
            let default_path = default_settings.install_path.clone().unwrap().as_str();
            if !Path::new(default_path).exists() {
                info!("Creating default path: {}", default_path);
                fs::create_dir_all(default_path).expect("error creating default path");
            }
            serialized_settings
        }
    };

    // Try to deserialize the settings
    let deserialized = serde_json::from_value::<AppSettings>(res).map_err(|e| e.to_string())?;

    // Print the deserialized settings for debugging
    println!("{:?}", deserialized);

    // Return the deserialized settings
    Ok(deserialized)
}

/// Save the app settings to the store
pub async fn save_settings(app: AppHandle, settings: AppSettings) -> Result<(), String> {
    // Check if the install path is empty
    if let Some(install_path) = &settings.install_path {
        if install_path.is_empty() {
            return Err("Install path cannot be empty".into());
        } else {
            // Check if the install path is valid
            let path = PathBuf::from(install_path);
            if !path.exists() {
                 return Err("Install path does not exist".to_string());
            }
            // Check if the new path is different from the old path
            let old_settings = read_settings_command(app.clone()).await?;
            if let Some(old_install_path) = old_settings.install_path {
                if !old_install_path.eq(install_path) {
                    let path = std::env::current_exe().map_err(|_| "unable to get current exe")?;
                    let cmd = Command::new("pkexec")
                        .arg(path.parent().unwrap().join("app_hub_backend"))
                        .arg("--action")
                        .arg("update")
                        .arg("--new-install-dir")
                        .arg(install_path)
                        .arg("--old-install-dir")
                        .arg(old_install_path)
                        .stdout(Stdio::piped())
                        .stderr(Stdio::piped())
                        .spawn();

                    match cmd {
                        Ok(mut child) => {
                            // Leggi lo stdout
                            if let Some(stdout) = child.stdout.take() {
                                let reader = BufReader::new(stdout);
                                for line in reader.lines() {
                                    if let Ok(line) = line {
                                        info!("app_hub_backend output: {}", line);
                                    }
                                    else {
                                        warn!("Failed to read app_hub_backend output");
                                    }
                                }
                            }

                            // Leggi lo stderr
                            if let Some(stderr) = child.stderr.take() {
                                let reader = BufReader::new(stderr);
                                for line in reader.lines() {
                                    if let Ok(line) = line {
                                        error!("app_hub_backend error: {}", line); // Stampalo come errore nel logger
                                    }
                                    else {
                                        warn!("Failed to read app_hub_backend error");
                                    }
                                }
                            }

                            let output = child.wait_with_output().expect("Failed to wait on child");
                            if output.status.success() {
                                info!("Installation successful");
                            } else {
                                error!("Installation failed");
                                return Err("Installation failed".to_string());
                            }
                        }
                        Err(error) => {
                            error!("error: {:?}", error);
                            return Err("Failed to install AppImage".to_string());
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
