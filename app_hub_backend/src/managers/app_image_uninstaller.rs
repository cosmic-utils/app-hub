use log::{error, info, warn};
use common_utils::app_images_helpers::find_icons_paths;
use common_utils::desktop_file_helpers::{delete_desktop_file_by_name, find_desktop_entry};
use common_utils::file_system_helpers::rm_file;

pub fn uninstall_app_image(app_name: String) -> Result<(), String> {
    info!("Uninstalling AppImage with app name: {}", app_name);

    let desktop_entry = match find_desktop_entry(app_name.clone()) {
        Ok(path) => path,
        Err(err) => {
            return Err(err);
        }
    };

    // Remove the AppImage
    if let Err(err) = rm_file(&desktop_entry.exec) {
        error!("{}", err);
        return Err("Failed to remove AppImage".into());
    }

    // Remove the desktop entry
    if let Err(err) = delete_desktop_file_by_name(&app_name) {
        error!("{}", err);
        return Err("Failed to remove desktop entry".into());
    }

    // Remove icons
    let icons = find_icons_paths(&desktop_entry.icon);

    for icon in icons {
        info!("Removing icon: {:?}", icon);
        if let Err(err) = rm_file(&icon) {
            warn!("{}", err);
        }
    }

    Ok(())
}