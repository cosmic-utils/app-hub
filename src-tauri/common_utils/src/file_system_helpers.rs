use std::{fs, io};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::Command;
use log::info;

/// This function is used to remove a file from the filesystem (used to remove AppImages and icons)
/// It returns a boolean indicating if the file was removed successfully
pub fn rm_file(file_path: &String) -> Result<bool, String> {
    match fs::remove_file(file_path) {
        Ok(_) => {
            info!("File removed successfully");
            Ok(true)
        },
        Err(e) => Err(format!("Failed to remove file: {}", e)),
    }
}

/// This function is used to remove a directory and all its contents from the filesystem
pub fn rm_dir_all(dir_path: &str) -> Result<bool, String> {
    match fs::remove_dir_all(dir_path) {
        Ok(_) => {
            info!("Directory removed successfully");
            Ok(true)
        },
        Err(e) => Err(format!("Failed to remove directory: {}", e)),
    }
}

/// This function is used to copy a directory and all its contents to a new location
pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
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

/// This function is used to write a file to the filesystem using sudo
pub fn sudo_write_file(file_path: &str, content: &str) -> Result<(), String> {
    let output = Command::new("pkexec")
        .arg("sh")
        .arg("-c")
        .arg(format!("echo '{}' > {}", content, file_path))
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        Ok(())
    } else {
        Err(format!("Failed to write file: {}", String::from_utf8_lossy(&output.stderr)))
    }
}

/// This function is used to remove a file from the filesystem using sudo
pub fn sudo_remove_file(file_path: &str) -> Result<(), String> {
    let output = Command::new("pkexec")
        .arg("rm")
        .arg(file_path)
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        Ok(())
    } else {
        Err(format!("Failed to remove file: {}", String::from_utf8_lossy(&output.stderr)))
    }
}

/// Find a .desktop file in the given directory
pub fn find_desktop_file_in_dir(dir_path: &str) -> Result<String, String> {
    let dir = Path::new(dir_path);
    let entries = match fs::read_dir(dir) {
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
            return Ok(path.to_string_lossy().to_string());
        }
    }

    Err("No desktop file found".to_string())
}

/// Add executable permission to a file
pub fn add_executable_permission(file_path: &str) {
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