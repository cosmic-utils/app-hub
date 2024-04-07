use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

use log::{debug, error, info};

use crate::helpers::file_system_helper::{copy_dir_all};

/// Install an AppImage file using the given file path
pub fn install_app_image(file_path: &String, installation_path: &String) -> Result<String, String> {
    // Try to create the directory and handle the error if it already exists
    match fs::create_dir(&installation_path) {
        Ok(_) => {}
        Err(ref e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
            info!("Directory already exists");
        }
        Err(e) => return Err(format!("Failed to create directory: {}", e)),
    }

    let path_buf = std::path::PathBuf::from(file_path);
    let file_name = path_buf.file_name().expect("Failed to get file name");
    // Define the destination path (installation path + file name)
    let dest_path = std::path::PathBuf::from(installation_path).join(file_name);

    let res = fs::copy(file_path, &dest_path).expect("Failed to copy file");

    // Set the executable permission to the file
    let mut perms = fs::metadata(&dest_path)
        .expect("Failed to get metadata")
        .permissions();
    // Set the executable permission to the file
    perms.set_mode(0o755);
    fs::set_permissions(&dest_path, perms).expect("Failed to set permissions");

    info!("Check file exist result: {:?}", res);
    info!("Cp result: {:?}", res);

    Ok("Installation successful".to_string())
}

/// Extract the .desktop file from the AppImage
pub fn app_image_extract_desktop_file(app_image_path: &str, app_name: &str) -> Result<(), &'static str> {
    let output = std::process::Command::new("bash")
        .arg("-c")
        .arg(format!("cd {} && ./{} --appimage-extract *.desktop", app_image_path, app_name))
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        Ok(())
    } else {
        let err = String::from_utf8_lossy(&output.stderr);
        error!("Failed to extract AppImage desktop file: {}", err);
        Err("Failed to extract AppImage icon")
    }
}

/// Extract all the files from the AppImage
pub fn app_image_extract_all(app_image_path: &str, app_name: &str) -> Result<(), &'static str> {
    let output = std::process::Command::new("bash")
        .arg("-c")
        .arg(format!("cd {} && ./{} --appimage-extract", app_image_path, app_name))
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        Ok(())
    } else {
        let err = String::from_utf8_lossy(&output.stderr);
        error!("Failed to extract AppImage desktop file: {}", err);
        Err("Failed to extract AppImage icon")
    }
}

// Install the icons from the AppImage by moving them to the system icons folder
pub fn install_icons(
    squashfs_root_path: &str
) -> Result<(), &'static str> {
    let icons_paths = [
        "/share/icons/hicolor/22x22/apps/",
        "/share/icons/hicolor/24x24/apps/",
        "/share/icons/hicolor/32x32/apps/",
        "/share/icons/hicolor/48x48/apps/",
        "/share/icons/hicolor/64x64/apps/",
        "/share/icons/hicolor/128x128/apps/",
        "/share/icons/hicolor/256x256/apps/",
        "/share/icons/hicolor/512x512/apps/",
        "/share/icons/hicolor/scalable/apps/",
        "/usr/share/icons/hicolor/22x22/apps/",
        "/usr/share/icons/hicolor/24x24/apps/",
        "/usr/share/icons/hicolor/32x32/apps/",
        "/usr/share/icons/hicolor/48x48/apps/",
        "/usr/share/icons/hicolor/64x64/apps/",
        "/usr/share/icons/hicolor/128x128/apps/",
        "/usr/share/icons/hicolor/256x256/apps/",
        "/usr/share/icons/hicolor/512x512/apps/",
        "/usr/share/icons/hicolor/scalable/apps/",
    ];

    for icon_path in icons_paths.iter() {
        let path = Path::new(squashfs_root_path).join(icon_path);
        if path.exists() {
            // non va bene perchè deve essere creato con lo stesso percorso di sopra
            // mettere icon_path va bene? funziona?
            match copy_dir_all(&path, Path::new(icon_path)) {
                Ok(copied) => {
                    info!("Copied: {:?}", copied);
                }
                Err(error) => {
                    error!("Error coping icon: {:?}", error);
                }
            };

        }
    }

    Ok(())
}

