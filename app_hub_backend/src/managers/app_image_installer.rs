use std::fs::File;
use std::io::Write;
use log::info;
use common_utils::app_images_helpers::{app_image_extract_squashroot, install_icons};
use common_utils::desktop_file_builder::DesktopFileBuilder;
use common_utils::desktop_file_helpers::find_desktop_file_location;
use common_utils::file_system_helpers::{add_executable_permission, find_desktop_file_in_dir, get_file_name};

pub fn install_app_image(file_path: String, installation_dir: String, no_sandbox: bool) -> Result<(), String> {
    info!("##### REQUESTED TO INSTALL APP ####");
    info!("# File path: {:?}", &file_path);
    info!("# No sandbox: {:?}", &no_sandbox);
    info!("#################################");

    // Add executable permission to the AppImage
    add_executable_permission(&file_path);

    let installation_file_name = get_file_name(&file_path)?;

    // extract squashrootfs from AppImage
    let squashroot_path = app_image_extract_squashroot(&file_path)?;


    // parse AppImage desktop file
    let desktop_file_path = match find_desktop_file_in_dir(
        &squashroot_path
    ) {
        Ok(path) => {
            info!("Desktop file found at: {:?}", path);
            path
        }
        Err(err) => {
            return Err(err);
        }
    };

    let mut desktop_builder = match DesktopFileBuilder::from_desktop_entry_path(
        &desktop_file_path,
        false
    ) {
        Ok(db) => {
            db
        }
        Err(err) => {
            return Err(err.to_string());
        }
    };

    // copy icons to icons directory
    info!("Installing icons...");
    if let Err(e) = install_icons(&squashroot_path) {
        return Err(e.to_string());
    }

    // set builder properties
    desktop_builder.set_exec(format!(
        "{}/{}",
        installation_dir.clone(),
        installation_file_name
    ));

    desktop_builder.set_path(installation_dir.to_string());

    if no_sandbox {
        desktop_builder.set_no_sandbox(true);
    }

    // write desktop file to /usr/share/applications
    let desktop_files_system_location = find_desktop_file_location()?;
    let app_name = match desktop_builder.name() {
        None => {return Err("Failed to get app name".to_string());}
        Some(name) => name
    };
    let desktop_entry_path = format!("{}/{}.desktop", desktop_files_system_location.to_string_lossy().to_string(), app_name);

    let desktop_file_content = match desktop_builder.generate_content_string() {
        Ok(content) => content,
        Err(e) => return Err(e.to_string())
    };

    info!("Writing .desktop file to: {:?}", desktop_entry_path);
    let mut file = File::create(desktop_entry_path).map_err(|e| "Failed to create .desktop file")?;
    file.write_all(desktop_file_content.as_bytes()).map_err(|_| "Failed to write .desktop file")?;

    // Copy the AppImage to the installation directory
    let installation_path = format!("{}/{}", installation_dir, installation_file_name);
    std::fs::copy(&file_path, &installation_path).map_err(|_| "Failed to copy AppImage to installation dir")?;

    // remove squashrootfs directory
    std::fs::remove_dir_all(&squashroot_path).map_err(|_| "Failed to remove squashroot dir")?;

    Ok(())
}
