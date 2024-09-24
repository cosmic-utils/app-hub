use fs_extra::dir;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::desktop_file_builder::DesktopFileBuilder;
use crate::desktop_file_helpers::find_desktop_file_location;
use crate::file_system_helpers::add_executable_permission;
use crate::models::installed_app::InstalledApp;
use log::{debug, error, info};

/// Install an AppImage file using the given file path
pub fn install_app_image_from_path(
    file_path: &String,
    installation_path: &String,
) -> Result<String, String> {
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
    add_executable_permission(&dest_path);

    info!("Check file exist result: {:?}", res);
    info!("Cp result: {:?}", res);

    Ok("Installation successful".to_string())
}

/// Extract the .desktop file from the AppImage
/// Returns the path to the extracted .desktop file
pub fn app_image_extract_squashroot(app_image_path: &PathBuf) -> Result<PathBuf, &'static str> {
    info!("Starting extraction of .desktop file from AppImage...");

    if !app_image_path.exists() || !app_image_path.is_file() {
        error!("AppImage file does not exist or is not a file");
        return Err("AppImage file does not exist or is not a file");
    }

    let app_name = match app_image_path.file_name() {
        None => return Err("Failed to get AppImage file name"),
        Some(name) => name
            .to_str()
            .ok_or_else(|| "Failed to convert OsStr to str")?,
    };

    // Get parent directory of app_image_path
    let parent_dir = match app_image_path.parent() {
        None => {
            error!("Failed to get parent directory of AppImage file");
            return Err("Failed to get parent directory of AppImage file");
        }
        Some(dir) => dir,
    };

    let command = format!(
        "cd {} && ./{} --appimage-extract",
        parent_dir.to_str().unwrap(),
        app_name
    );
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

/// Install the icons from the AppImage by moving them to the installation dir icons folder
pub fn choose_icon(
    squashfs_root_path: &PathBuf,
    installation_path: &PathBuf,
) -> Result<PathBuf, &'static str> {
    // Find all icons file
    let image_files = find_image_files(squashfs_root_path);
    debug!("Found {} image files", image_files.len());
    debug!("Image files: {:?}", image_files);
    if image_files.len() == 0 {
        return Err("No image files found");
    }

    // Choose the biggest image file
    let biggest_image = image_files
        .iter()
        .max_by_key(|f| f.metadata().unwrap().len())
        .unwrap();

    debug!("Biggest image file: {:?}", biggest_image);

    // Check if the icons directory exists or create it
    let icons_dir = installation_path.join("icons");
    match fs::create_dir(&icons_dir) {
        Ok(_) => {
            info!("Created icons directory: {:?}", icons_dir);
        }
        Err(ref e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
            info!("Icons directory already exists");
        }
        Err(e) => {
            error!("Failed to create icons directory: {}", e);
            return Err("Failed to create icons directory");
        }
    }

    // Copy the biggest image file into the installation path
    let icon_path = installation_path
        .join("icons")
        .join(biggest_image.file_name().unwrap());
    match fs::copy(biggest_image, &icon_path) {
        Ok(_) => {
            info!("Copied icon to: {:?}", icon_path);
        }
        Err(e) => {
            error!("Failed to copy icon: {}", e);
            return Err("Failed to copy icon");
        }
    }

    Ok(icon_path)
}

fn recursive_copy(source: &Path, destination: &Path) -> Result<(), &'static str> {
    let mut options = dir::CopyOptions::new();
    options.overwrite = true;
    match dir::copy(source, destination, &options) {
        Ok(copied) => {
            info!("Copied {} icons", copied);
        }
        Err(error) => {
            error!("Failed to copy icons: {}", error);
            return Err("Failed to copy icons");
        }
    }
    Ok(())
}

pub fn remove_icon(icon_path: &PathBuf) -> Result<(), &'static str> {
    if icon_path.exists() {
        match fs::remove_file(icon_path) {
            Ok(_) => {
                info!("Removed icon: {:?}", icon_path);
            }
            Err(e) => {
                error!("Failed to remove icon: {}", e);
                return Err("Failed to remove icon");
            }
        }
    } else {
        info!("Icon does not exist: {:?}", icon_path);
    }

    Ok(())
}

/// Find all the image files in the given directory
fn find_image_files(dir: &PathBuf) -> Vec<PathBuf> {
    let mut image_files = Vec::new();
    let extensions = vec!["png", "jpg", "jpeg", "svg"];

    for entry in WalkDir::new(dir) {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            if let Some(ext) = entry.path().extension() {
                if extensions.contains(&ext.to_str().unwrap()) {
                    image_files.push(entry.path().to_path_buf());
                }
            }
        }
    }

    image_files
}

/// Detect all installed app images and reads it's metadata
pub fn read_all_app() -> Result<Vec<InstalledApp>, String> {
    let mut apps: Vec<InstalledApp> = Vec::new();

    // read all .desktop files in the applications directory
    let applications_dir: PathBuf = find_desktop_file_location().map_err(|e| e.to_string())?;

    match std::fs::read_dir(applications_dir) {
        Ok(entries) => {
            for entry in entries {
                let entry = entry.as_ref().unwrap();
                let desktop_file = DesktopFileBuilder::from_desktop_entry_path(&entry.path(), true);
                if desktop_file.is_err() {
                    error!(
                        "Failed to read desktop file: {}",
                        desktop_file.err().unwrap()
                    );
                    continue;
                }

                let desktop_entry = desktop_file.unwrap();

                apps.push(InstalledApp {
                    name: desktop_entry.name().unwrap(),
                    app_path: desktop_entry.exec().unwrap(),
                    version: desktop_entry.version(),
                    categories: desktop_entry.categories(),
                    icon_path: Some(PathBuf::from(desktop_entry.icon().unwrap())),
                });
            }
        }
        Err(err) => {
            return Err(format!("Failed to read directory: {}", err));
        }
    }

    Ok(apps)
}
