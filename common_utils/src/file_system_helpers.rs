use log::{error, info};
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::{fs, io};

/// This function is used to remove a file from the filesystem (used to remove AppImages and icons)
/// It returns a boolean indicating if the file was removed successfully
pub fn rm_file(file_path: &String) -> Result<bool, String> {
    match fs::remove_file(file_path) {
        Ok(_) => {
            info!("File removed successfully");
            Ok(true)
        }
        Err(e) => Err(format!("Failed to remove file: {}", e)),
    }
}

/// This function is used to remove a directory and all its contents from the filesystem
pub fn rm_dir_all(dir_path: &str) -> Result<bool, String> {
    match fs::remove_dir_all(dir_path) {
        Ok(_) => {
            info!("Directory removed successfully");
            Ok(true)
        }
        Err(e) => Err(format!("Failed to remove directory: {}", e)),
    }
}

/// This function is used to copy a directory and all its contents to a new location
pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    if let Err(err) = fs::create_dir_all(&dst) {
        error!("Failed to create directory: {}", err);
        return Err(err);
    }
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

/// Find a .desktop file in the given directory
pub fn find_desktop_file_in_dir(dir_path: &PathBuf) -> Result<PathBuf, String> {
    let entries = match fs::read_dir(dir_path.as_path()) {
        Ok(entries) => entries,
        Err(e) => return Err(format!("Failed to read directory: {}", e)),
    };

    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => return Err(format!("Failed to read entry: {}", e)),
        };
        let path = entry.path();
        let extension = match path.extension() {
            Some(ext) => ext,
            None => continue,
        };
        if extension == "desktop" {
            return Ok(path);
        }
    }
    Err("No desktop file found".to_string())
}

/// Add executable permission to a file
pub fn add_executable_permission(file_path: &PathBuf) {
    // Set the executable permission to the file
    let mut perms = fs::metadata(&file_path)
        .expect("Failed to get metadata")
        .permissions();
    // Set the executable permission to the file
    perms.set_mode(0o755);
    fs::set_permissions(&file_path, perms).expect("Failed to set permissions")
}

/// Check if a directory is empty
pub fn is_directory_empty(dir_path: &Path) -> io::Result<bool> {
    let mut entries = fs::read_dir(dir_path)?;
    Ok(entries.next().is_none())
}

/// Get the file name from a path
pub fn get_file_name(file_path: &PathBuf) -> Result<String, String> {
    match file_path.file_name() {
        Some(file_name) => Ok(file_name.to_string_lossy().to_string()),
        None => Err("Failed to get file name".to_string()),
    }
}
