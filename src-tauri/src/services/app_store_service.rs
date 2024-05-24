use std::path::PathBuf;
use log::{debug, error, info};
use regex::Regex;
use serde::de;
use serde_json::Value;

use tauri::{AppHandle, Manager, Wry};
use tauri_plugin_store::{StoreCollection, with_store};
use crate::models::app_database::AppDatabase;
use crate::models::response_app_hub_data::ResponseAppHubData;
use tauri_plugin_http::reqwest;
use crate::models::app_settings::AppSettings;
use crate::models::request_installation::RequestInstallation;
use crate::services::app_image_service::install_app_image;
use crate::services::settings_service::read_settings;

const APP_HUB_URL: &str = "https://api.github.com/repos/AppImage/appimage.github.io/contents/data";
const USER_AGENT: &str = "AppImageLauncher";
const FETCH_ERROR: &str = "Failed to fetch data from AppHub";

pub async fn update_app_database(app: AppHandle) -> Result<(), String> {
    info!("Updating database");
    // Get the store collection from the app state
    let stores = app.state::<StoreCollection<Wry>>();
    // Define the path to the settings file
    let path = PathBuf::from("app_database.bin");

    let database = match fetch_all_apps_from_app_hub().await {
        Ok(data) => {
            data
        }
        Err(err) => {
            debug!("Failed to fetch data from AppHub: {}", err);
            AppDatabase {
                app_list: Vec::new()
            }
        }
    };

    let deserialized_database = serde_json::to_value(&database).map_err(|e| e.to_string())?;
    with_store(app.clone(), stores, path, |store| {
        store.insert("app_database".to_string(), deserialized_database)
    }).expect("error saving default settings");
    serde_json::to_value(database).map_err(|e| e.to_string())?;

    Ok(())
}

pub async fn get_app_list(app: AppHandle) -> Result<AppDatabase, String> {
    let stores = app.state::<StoreCollection<Wry>>();
    let path = PathBuf::from("app_database.bin");

    let database = with_store(app.clone(), stores.clone(), path.clone(), |store| {
        // Try to get the settings from the store
        match store.get("app_database") {
            // If the settings are found, clone them and return
            Some(value) => Ok(Some(value.clone())),
            // If the settings are not found, return None
            None => Ok(None),
        }
    }).map_err(|e| e.to_string())?;

    match database {
        None => {
            return Err(String::from("No database found"));
        }
        Some(content) => {
            let deserialized_database = serde_json::from_value::<AppDatabase>(content).map_err(|e| e.to_string())?;
            Ok(deserialized_database)
        }
    }
}

pub async fn install_app_from_remote(app: AppHandle, download_url: String, app_name: String) -> Result<(), String> {
    info!("Installing app from remote");

    let path = match download_app_image(download_url, app_name).await {
        Ok(downloaded_to_path) => {
            info!("Downloaded app to: {:?}", downloaded_to_path);
            downloaded_to_path
        }
        Err(error) => {
            error!("Failed to download file: {}", error);
            return Err(error);
        }
    };
    
    let request_installation = RequestInstallation {
        file_path: path.to_string_lossy().to_string(),
        no_sandbox: Some(false),
    };
    
    if let Err(err) = install_app_image(app.clone(), request_installation).await {
        error!("Failed to install app: {}", err);
        return Err(err);
    }

    // remove temp file
    std::fs::remove_file(path).map_err(|e| e.to_string())?;

    Ok(())
}

async fn download_app_image(url: String, app_name: String) -> Result<PathBuf, String> {
    let client = reqwest::Client::builder().user_agent(USER_AGENT).build().map_err(|e| e.to_string())?;
    let res = client.get(url).send().await.map_err(|e| e.to_string())?;

    if !res.status().is_success() {
        return Err(String::from("Failed to download file"));
    }

    let text_response = res.text().await.map_err(|e| e.to_string())?;
    let url = extract_first_link(text_response).map_err(|e| e.to_string())?;
    info!("Downloading app from remote: {}", url);

    let res = client.get(url).send().await.map_err(|e| e.to_string())?;

    let bytes = res.bytes().await.map_err(|e| e.to_string())?;

    let temp_dir = std::env::temp_dir();
    let target_path = temp_dir.join(format!("{}.AppImage", app_name));

    info!("Writing app to: {:?}", target_path);
    std::fs::write(&target_path, bytes).map_err(|e| e.to_string())?;

    Ok(target_path)
}

fn extract_first_link(text: String) -> Result<String, String> {
    let first = text.split('\n').next();
    if first.is_none() {
        return Err(String::from("No links found"));
    }
    Ok(first.unwrap().to_string())
}

async fn fetch_all_apps_from_app_hub() -> Result<AppDatabase, String> {
    let client = reqwest::Client::builder().user_agent(USER_AGENT).build().map_err(|e| e.to_string())?;
    let res = client.get(APP_HUB_URL).send().await.map_err(|e| e.to_string())?;

    if res.status().is_success() {
        let text_response = res.text().await.map_err(|e| e.to_string())?;
        let deserialized_res: ResponseAppHubData = deserialize::<ResponseAppHubData>(&text_response)?;

        Ok(AppDatabase{
            app_list: deserialized_res,
        })
    } else {
        debug!("Failed to fetch data from AppHub with status: {:?}", res.status());
        Err(String::from(FETCH_ERROR))
    }
}

fn deserialize<T: de::DeserializeOwned>(to_deserialize: &String) -> Result<T, String> {
    match serde_json::from_str::<T>(&to_deserialize) {
        Ok(deserialized) => {
            Ok(deserialized)
        }
        Err(err) => {
            Err(String::from(err.to_string()))
        }
    }
}