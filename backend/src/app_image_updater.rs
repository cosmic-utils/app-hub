use common_utils::desktop_file_builder::DesktopFileBuilder;
use common_utils::desktop_file_helpers::find_desktop_entries_by_exec_contains;
use common_utils::file_system_helpers::copy_dir_all;
use log::{error, info, warn};
use std::fs::{remove_dir_all, File};
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

/// Update the image of an app
pub fn app_image_update(
    old_install_dir: String,
    new_install_dir: String,
) -> Result<(), String> {
    if old_install_dir.eq(&new_install_dir) {
        return Err("Old and new install directories are the same".into());
    }

    info!(
        "Updating app images from {} to {}",
        old_install_dir, new_install_dir
    );

    if let Err(e) = copy_dir_all(&old_install_dir, &new_install_dir) {
        return Err(format!("Failed to copy app images: {}", e));
    }

    info!("App images moved successfully");

    if let Err(e) = remove_dir_all(&old_install_dir) {
        return Err(format!("Failed to delete old install directory: {}", e));
    }

    info!("Old install directory deleted successfully");

    let desktop_entries = match find_desktop_entries_by_exec_contains(&old_install_dir) {
        Ok(entries) => entries,
        Err(e) => return Err(format!("Failed to find desktop entries: {}", e)),
    };

    for desktop_entry in desktop_entries {
        let mut desktop_file_builder =
            match DesktopFileBuilder::from_desktop_entry_path(&PathBuf::from(&desktop_entry), true)
            {
                Ok(builder) => builder,
                Err(e) => {
                    info!("Failed to parse desktop entry: {}", e);
                    continue;
                }
            };

        let new_icon = desktop_file_builder
            .icon()
            .unwrap()
            .replace(&old_install_dir, &new_install_dir);

        desktop_file_builder.set_icon(new_icon);

        // Check exec field
        if desktop_file_builder.exec().is_none() {
            info!(
                "Failed to parse desktop entry (exec missing): {}",
                &desktop_entry
            );
            continue;
        }

        let new_exec = desktop_file_builder
            .exec()
            .unwrap()
            .replace(&old_install_dir, &new_install_dir);
        info!("New exec: {}", new_exec);
        desktop_file_builder.set_exec(new_exec);

        // Update path fields
        if desktop_file_builder.path().is_some() {
            desktop_file_builder.set_path(
                desktop_file_builder
                    .path()
                    .unwrap()
                    .replace(old_install_dir.as_str(), new_install_dir.as_str()),
            );
        }
        match desktop_file_builder.generate_content_string() {
            Ok(content) => {
                let mut file = File::create(&desktop_entry);
                match file {
                    Ok(mut file) => {
                        file.write_all(content.as_bytes())
                            .expect("Error updating .desktop file");
                        if let Err(error) = file.write_all(content.as_bytes()) {
                            error!("Error writing .desktop file: {}", error);
                        }
                    }
                    Err(error) => {
                        error!("Error opening .desktop file: {}", error);
                    }
                }
            }
            Err(error) => {
                warn!("Error updating .desktop file: {}", error);
            }
        }
    }

    // Add executable permissions to all app images
    if let Err(e) = Command::new("chmod")
        .arg("-R")
        .arg("+x")
        .arg(&new_install_dir)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        return Err(format!(
            "Failed to add executable permissions to app images: {}",
            e
        ));
    }

    Ok(())
}
