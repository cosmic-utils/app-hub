use std::fs;
use std::path::{Path, PathBuf};
use glob::glob;

use log::{debug, error, info};
use crate::file_system_helpers::{add_executable_permission};

/// Install an AppImage file using the given file path
pub fn install_app_image_from_path(file_path: &String, installation_path: &String) -> Result<String, String> {
    // Try to create the directory and handle the error if it already exists
    match fs::create_dir(&installation_path) {
        Ok(_) => {}
        Err(ref e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
            info!("Directory already exists");
        }
        Err(e) => return Err(format!("Failed to create directory: {}", e))
    }

    let path_buf = std::path::PathBuf::from(file_path);
    let file_name = path_buf.file_name().expect("Failed to get file name");
    // Define the destination path (installation path + file name)
    let dest_path = std::path::PathBuf::from(installation_path).join(file_name);

    let res = fs::copy(file_path, &dest_path).expect("Failed to copy file");

    // Set the executable permission to the file
    add_executable_permission(&dest_path.to_str().unwrap().to_string());

    info!("Check file exist result: {:?}", res);
    info!("Cp result: {:?}", res);

    Ok("Installation successful".to_string())
}

/// Extract the .desktop file from the AppImage
/// Returns the path to the extracted .desktop file
pub fn app_image_extract_squashroot(app_image_path: &str) -> Result<PathBuf, &'static str> {
    info!("Starting extraction of .desktop file from AppImage...");

    let app_image_path = Path::new(app_image_path);

    if !app_image_path.exists() || !app_image_path.is_file() {
        error!("AppImage file does not exist or is not a file");
        return Err("AppImage file does not exist or is not a file");
    }

    let app_name = match app_image_path.file_name() {
        None => return Err("Failed to get AppImage file name"),
        Some(name) => {
            name.to_str().ok_or_else(|| "Failed to convert OsStr to str")?
        }
    };

    // Get parent directory of app_image_path
    let parent_dir = match app_image_path.parent() {
        None => {
            error!("Failed to get parent directory of AppImage file");
            return Err("Failed to get parent directory of AppImage file");
        }
        Some(dir) => dir
    };


    let command = format!("cd {} && ./{} --appimage-extract", parent_dir.to_str().unwrap(), app_name);
    debug!("Running command: {}", command);

    let output = std::process::Command::new("bash")
        .arg("-c")
        .arg(command)
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        info!("Successfully extracted .desktop file from AppImage.");
    } else {
        let err = String::from_utf8_lossy(&output.stderr);
        error!("Failed to extract AppImage desktop file: {}", err);
        return Err("Failed to extract AppImage desktop file");
    }
    let squashfs_root_path = parent_dir.join(format!("squashfs-root"));
    Ok(squashfs_root_path)
}

// Install the icons from the AppImage by moving them to the system icons folder
pub fn install_icons(
    squashfs_root_path: &PathBuf
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
        if squashfs_root_path.exists() {
            if let Err(err) = fs::copy(squashfs_root_path.to_str().unwrap(), icon_path) {
                error!("Failed to copy icons in {}: {}", icon_path, err);
            }
        }
    }
    Ok(())
}

pub fn remove_icons() -> Result<(), &'static str> {
    let icons_paths = [
        "share/icons/hicolor/22x22/apps/",
        "share/icons/hicolor/24x24/apps/",
        "share/icons/hicolor/32x32/apps/",
        "share/icons/hicolor/48x48/apps/",
        "share/icons/hicolor/64x64/apps/",
        "share/icons/hicolor/128x128/apps/",
        "share/icons/hicolor/256x256/apps/",
        "share/icons/hicolor/512x512/apps/",
        "share/icons/hicolor/scalable/apps/",
        "usr/share/icons/hicolor/22x22/apps/",
        "usr/share/icons/hicolor/24x24/apps/",
        "usr/share/icons/hicolor/32x32/apps/",
        "usr/share/icons/hicolor/48x48/apps/",
        "usr/share/icons/hicolor/64x64/apps/",
        "usr/share/icons/hicolor/128x128/apps/",
        "usr/share/icons/hicolor/256x256/apps/",
        "usr/share/icons/hicolor/512x512/apps/",
        "usr/share/icons/hicolor/scalable/apps/",
    ];

    let mut sh_template_script = String::new();

    for icon_path in icons_paths.iter() {
        sh_template_script.push_str(&format!("rm -r {}{} 2>/dev/null  || : \n", "/usr/", icon_path));
    }

    debug!("Remove icons script: {}", sh_template_script);

    // Run the script with sudo
    let output = std::process::Command::new("pkexec")
        .arg("sh")
        .arg("-c")
        .arg(sh_template_script)
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        Ok(())
    } else {
        let err = String::from_utf8_lossy(&output.stderr);
        error!("Failed to remove icons: {}", err);
        Err("Failed to remove icons")
    }
}

/// Update the icon cache
pub fn update_icon_cache() -> Result<(), &'static str> {
    let output = std::process::Command::new("pkexec")
        .arg("gtk-update-icon-cache")
        .arg("-f")
        .arg("/usr/share/icons/hicolor")
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        Ok(())
    } else {
        let err = String::from_utf8_lossy(&output.stderr);
        error!("Failed to update icon cache: {}", err);
        Err("Failed to update icon cache")
    }
}

/// Find the icon paths for the given icon name
pub fn find_icons_paths(icon_name: &str) -> Vec<String> {
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

    let mut paths = Vec::new();

    for icon_path in icons_paths.iter() {
        let pattern = format!("{}{}.*", icon_path, icon_name);

        for entry in glob(&pattern).expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => {
                    if path.is_file() {
                        paths.push(path.to_str().unwrap().to_string());
                    }
                },
                Err(e) => println!("{:?}", e),
            }
        }
    }


    paths
}
