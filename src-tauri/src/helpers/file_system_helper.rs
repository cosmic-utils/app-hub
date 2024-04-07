use std::{fs, io};
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use mime_guess::from_path;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::Command;
use log::{debug, info};

/// This function is used to convert an image file to a Base64 string
/// It returns a Result containing the Base64 string or an error
/// The function takes a path to the image file as an argument
pub fn image_to_base64(path: &str) -> Result<String, std::io::Error> {
    // Open the file in read mode
    let mut file = File::open(path)?;

    // Create a vector to hold the image bytes
    let mut buffer = Vec::new();

    // Read the file into the byte array
    file.read_to_end(&mut buffer)?;

    // Encode the byte array into a Base64 string using the STANDARD engine
    let encoded = STANDARD.encode(&buffer);

    // Determine the MIME type of the file
    let mime = from_path(path).first_or_octet_stream();
    let mime_type = mime.as_ref();

    // Add the correct header to the encoded string
    let result = format!("data:{};base64,{}", mime_type, encoded);

    Ok(result)
}

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
