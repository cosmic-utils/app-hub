use common_utils::app_images_helpers::remove_icon;
use common_utils::desktop_file_helpers::{delete_desktop_file_by_name, find_desktop_entry};
use common_utils::file_system_helpers::rm_file;
use log::{error, info};
use std::path::PathBuf;

pub fn uninstall_app_image(app_name: String) -> Result<(), String> {
    info!("Uninstalling AppImage with app name: {}", app_name);

    let desktop_entry = match find_desktop_entry(app_name.clone()) {
        Ok(path) => path,
        Err(err) => {
            return Err(err);
        }
    };

    let exec = &desktop_entry
        .exec
        .replace("--no-sandbox", "")
        .trim()
        .to_string();

    // Remove the AppImage
    if let Err(err) = rm_file(exec) {
        error!(
            "Failed to remove app image file at {}: {}",
            desktop_entry.exec, err
        );
        return Err("Failed to remove AppImage".into());
    }

    // Remove the desktop entry
    if let Err(err) = delete_desktop_file_by_name(&app_name) {
        error!("Failed to remove desktop entry: {}", err);
        return Err("Failed to remove desktop entry".into());
    }

    // Remove icons
    if let Err(err) = remove_icon(&PathBuf::from(&desktop_entry.icon)) {
        error!("Failed to remove icons: {}", err);
    }

    Ok(())
}
