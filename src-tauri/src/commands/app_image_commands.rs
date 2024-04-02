use dirs;
use log::{error, info, warn};
use tauri::AppHandle;
use crate::commands::app_settings_commands::read_settings;

use crate::helpers::app_images_helpers::{copy_icon_file, install_app_image};
use crate::helpers::desktop_file_builder::DesktopFileBuilder;
use crate::helpers::desktop_file_helpers::{delete_desktop_file_by_name, find_desktop_entry, read_all_app};
use crate::helpers::file_system_helper::rm_file;
use crate::models::app_list::{App, AppList};
use crate::models::request_installation::RequestInstallation;

#[tauri::command]
pub async fn install_app(app: AppHandle, request_installation: RequestInstallation) -> Result<String, String> {
    info!("Installing file: {:?}", request_installation);

    info!("### REQUESTED TO INSTALL APP ###");
    info!("File path: {:?}", request_installation.file_path);
    info!("Icon path: {:?}", request_installation.icon_path);
    info!("App name: {:?}", request_installation.app_name);
    info!(
        "App description: {:?}",
        request_installation.app_description
    );
    info!("App type: {:?}", request_installation.app_type);
    info!("Terminal: {:?}", request_installation.terminal);
    info!("#################################");

    let settings = match read_settings(app).await {
        Ok(settings) => settings,
        Err(err) => {
            error!("{}", err);
            return Err(err);
        }
    };

    match install_app_image(&request_installation.file_path, settings.install_path.as_ref().unwrap()) {
        Ok(_) => {
            info!("AppImage installation successful");
        }
        Err(err) => {
            return Err(err);
        }
    }

    let path_buf = std::path::PathBuf::from(&request_installation.file_path);
    let file_name = path_buf.file_name().expect("Failed to get file name");

    let icon_path = match copy_icon_file(&request_installation.icon_path, settings.install_path.as_ref().unwrap()) {
        Ok(path) => {
            info!("Icon file copied to: {:?}", path);
            path
        }
        Err(err) => {
            return Err(err);
        }
    };

    let mut desktop_builder = DesktopFileBuilder::new();

    // Set mandatory fields
    if request_installation.app_type.is_some() {
        desktop_builder.set_type(request_installation.app_type.unwrap());
    } else {
        desktop_builder.set_type("Application".to_string());
    }
    desktop_builder.set_version("1.0".to_string()); //TODO: make this settable by the user as advanced setting
    desktop_builder.set_name(request_installation.app_name.clone());
    desktop_builder.set_exec(format!(
        "{}/{}",
        settings.install_path.unwrap(),
        file_name.to_string_lossy()
    ));

    // Set optional fields
    desktop_builder.set_icon(icon_path);
    desktop_builder.set_terminal(request_installation.terminal.unwrap_or(false));
    //TODO: add categories to the settings
    //desktop_builder.set_categories("Utility".to_string());
    if request_installation.app_description.is_some() {
        desktop_builder.set_comment(request_installation.app_description.unwrap());
    }

    // Create destingation path
    let home_dir = dirs::home_dir().expect("Failed to get home directory");
    let desktop_entry_path = home_dir
        .join(".local")
        .join("share")
        .join("applications")
        .join(format!("{}.desktop", request_installation.app_name));

    // Set no sandbox
    if request_installation.no_sandbox.is_some() && request_installation.no_sandbox.unwrap() {
        info!("Setting no sandbox");
        desktop_builder.set_no_sandbox(true);
    }

    // Create the desktop entry
    match desktop_builder.write_to_file(desktop_entry_path.to_string_lossy().to_string()) {
        Ok(_) => {
            info!("Desktop entry created successfully");
        }
        Err(err) => {
            return Err(err);
        }
    }

    Ok("Installation successful".to_string())
}

#[tauri::command]
pub async fn read_app_list() -> Result<AppList, String> {
    let apps: Vec<App> = read_all_app()?;
    Ok(AppList { apps })
}

#[tauri::command]
pub async fn uninstall_app(app: App) -> Result<bool, String> {
    let desktop_entry = match find_desktop_entry(app.name.clone()) {
        Ok(path) => path,
        Err(err) => {
            return Err(err);
        }
    };

    info!("Uninstalling app at: {:?}", &desktop_entry.exec);

    // Remove the AppImage
    let app_removed: bool = match rm_file(&desktop_entry.exec) {
        Ok(result) => {
            info!("AppImage removed successfully");
            result
        }
        Err(err) => {
            error!("{}", err);
            return Err(err);
        }
    };

    // Remove the icon
    let icon_removed: bool = match rm_file(&desktop_entry.icon_path) {
        Ok(result) => {
            info!("Icon removed successfully");
            result
        }
        Err(err) => {
            error!("{}", err);
            return Err(err);
        }
    };

    if !icon_removed {
        warn!("Failed to remove icon");
    }

    // Remove the desktop entry
    let desktop_removed: bool = match delete_desktop_file_by_name(&app.name) {
        Ok(result) => {
            info!("Desktop entry removed successfully");
            result
        }
        Err(err) => {
            error!("{}", err);
            return Err(err);
        }
    };

    if app_removed && desktop_removed {
        info!("App uninstalled successfully");
        Ok(true)
    } else {
        Err("Failed to remove app".to_string())
    }
}
