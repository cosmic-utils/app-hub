use log::{debug, error, info, warn};
use tauri::AppHandle;
use crate::commands::app_settings_commands::read_settings_command;
use crate::helpers::app_images_helpers::{app_image_extract_all, app_image_extract_desktop_file, find_icons_paths, install_app_image_from_path, install_icons, update_icon_cache};
use crate::helpers::desktop_file_builder::DesktopFileBuilder;
use crate::helpers::desktop_file_helpers::{delete_desktop_file_by_name, find_desktop_entry, find_desktop_file_location};
use crate::helpers::file_system_helper::{add_executable_permission, find_desktop_file_in_dir, image_to_base64, rm_dir_all, rm_file, sudo_remove_file};
use crate::models::app_list::App;
use crate::models::request_installation::RequestInstallation;

/// Install an AppImage
pub async fn install_app_image(app: AppHandle, request_installation: RequestInstallation) -> Result<(), String> {
    info!("##### REQUESTED TO INSTALL APP ####");
    info!("# File path: {:?}", request_installation.file_path);
    info!("# No sandbox: {:?}", request_installation.no_sandbox);
    info!("#################################");

    // Add executable permission to the AppImage
    add_executable_permission(&request_installation.file_path);

    // Read path where to install apps
    let apps_installation_path = match read_settings_command(app).await {
        Ok(settings) => settings.install_path.unwrap(),
        Err(err) => {
            error!("{}", err);
            return Err("Failed to read settings".to_string());
        }
    };

    // AppImage file path selected by the user
    let app_image_file_path = std::path::PathBuf::from(&request_installation.file_path);
    // AppImage file name
    let file_name = app_image_file_path.file_name().expect("Failed to get file name");
    // AppImage to install parent directory path
    let app_image_directory_path = app_image_file_path.parent().expect("Failed to get directory").to_path_buf();

    // Extract the AppImage .desktop file
    app_image_extract_desktop_file(
        app_image_directory_path.to_str().unwrap(),
        file_name.to_string_lossy().to_string().as_str()
    )?;

    // Squashfs-root directory path (app image extracted directory)
    let squashfs_path = std::path::PathBuf::from(app_image_directory_path.clone()).join("squashfs-root");

    // Find the desktop file in the squashfs-root directory
    let desktop_file_path = match find_desktop_file_in_dir(
        squashfs_path.to_string_lossy().to_string().as_str()
    ) {
        Ok(path) => {
            info!("Desktop file found at: {:?}", path);
            path
        }
        Err(err) => {
            return Err(err);
        }
    };

    // Parse the desktop file
    let mut desktop_builder = match DesktopFileBuilder::from_desktop_entry_path(
        desktop_file_path.to_string(),
        false
    ) {
        Ok(db) => {
            info!("Desktop entry parsed successfully");
            db
        }
        Err(error) => {
            return Err(error.to_string());
        }
    };

    // Extract all appImage content
    match app_image_extract_all(
        app_image_directory_path.to_str().unwrap(),
        file_name.to_string_lossy().to_string().as_str()
    ) {
        Ok(_res) => {
            info!("AppImage extracted successfully");
        }
        Err(error) => {
            error!("Error extracting all app image content: {}", error);
            return Err(error.to_string());
        }
    }

    // Move icons to the system icons folder
    match install_icons(
        squashfs_path.to_str().unwrap(),
    ) {
        Ok(_res) => {
            info!("Icon file copied");
        }
        Err(err) => {
            error!("{}", err);
            return Err(err.to_string());
        }
    }

    // Set mandatory fields
    desktop_builder.set_exec(format!(
        "{}/{}",
        apps_installation_path,
        file_name.to_string_lossy()
    ));

    // Set optional fields
    //desktop_builder.set_icon(copied_icon_path);

    // Create destination path
    let desktop_files_location = find_desktop_file_location()?;
    let desktop_files_location_path = std::path::PathBuf::from(desktop_files_location);
    //TODO: Check if the app name is present
    let desktop_entry_path = desktop_files_location_path
        .join(format!("{}.desktop", desktop_builder.name().unwrap()));

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
            return Err(err.to_string());
        }
    }

    // Install the AppImage
    match install_app_image_from_path(
        &app_image_file_path.to_str().unwrap().to_string(),
        &apps_installation_path
    ) {
        Ok(_res) => {
            info!("AppImage installed successfully");
        }
        Err(err) => {
            error!("{}", err);
            return Err(err.to_string());
        }
    }

    rm_dir_all(squashfs_path.to_str().unwrap()).expect("Failed to remove squashfs-root directory");

    // Update icons cache
    match update_icon_cache() {
        Ok(_res) => {
            info!("Icon cache updated successfully");
        }
        Err(error) => {
            error!("{}", error);
        }
    }

    Ok(())
}

/// Read all desktop files in the applications directory
/// This retrieved list contains only files installed by AppHub
pub fn read_all_app() -> Result<Vec<App>, String> {
    let mut apps: Vec<App> = Vec::new();

    // read all .desktop files in the applications directory
    let applications_dir: String = find_desktop_file_location().map_err(|e| e.to_string())?;

    match std::fs::read_dir(applications_dir) {
        Ok(entries) => {
            for entry in entries {
                let entry = entry.as_ref().unwrap();

                let desktop_file = DesktopFileBuilder::from_desktop_entry_path(entry.path().to_str().unwrap().to_string(), true);

                if desktop_file.is_err() {
                    error!("Failed to read desktop file: {}", desktop_file.err().unwrap());
                    continue;
                }

                let desktop_entry = desktop_file.unwrap();

                let icons_path = find_icons_paths(&desktop_entry.icon().unwrap());

                debug!("Reading icon at: {:?}", desktop_entry.icon());
                let base64_icon: Option<String> = if icons_path.len() > 0 {
                    match image_to_base64(icons_path.get(0).unwrap().as_str()) {
                        Ok(base64) => Some(base64),
                        Err(err) => {
                            info!("Failed to convert image to base64: {}", err);
                            None
                        }
                    }
                } else {
                    None
                };

                apps.push(App {
                    name: desktop_entry.name().unwrap(),
                    icon_base64: base64_icon,
                    app_path: desktop_entry.exec().unwrap(),
                    version: desktop_entry.version(),
                    categories: desktop_entry.categories(),
                });
            }
        }
        Err(err) => {
            return Err(format!("Failed to read directory: {}", err));
        }
    }

    Ok(apps)
}

/// Uninstall an AppImage
pub fn uninstall_app(app: App) -> Result<bool, String> {
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

    // Remove icons
    let icon_name: String = desktop_entry.icon;
    let icons = find_icons_paths(&icon_name.as_str());

    for icon in icons {
        info!("Removing icon: {:?}", icon);
        let icon_removed: bool = match sudo_remove_file(&icon) {
            Ok(_result) => {
                info!("Icon removed successfully");
                true
            }
            Err(err) => {
                error!("{}", err);
                false
            }
        };
        if !icon_removed {
            warn!("Failed to remove icon");
        }
    }

    if app_removed && desktop_removed {
        info!("App uninstalled successfully");
        Ok(true)
    } else {
        Err("Failed to remove app".to_string())
    }
}
