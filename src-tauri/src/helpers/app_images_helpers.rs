use std::fs;
use std::os::unix::fs::PermissionsExt;
use log::info;

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

/// Copy the icon file to the installation directory
pub fn copy_icon_file(icon_path: &String) -> Result<String, String> {
    let home_dir = dirs::home_dir().expect("Failed to get home directory");
    let icons_path = home_dir.join("AppImages").join("icons");

    // Try to create the directory and handle the error if it already exists
    match fs::create_dir(&icons_path) {
        Ok(_) => {}
        Err(ref e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
            info!("Directory already exists");
        }
        Err(e) => return Err(format!("Failed to create directory: {}", e)),
    }

    let path_buf = std::path::PathBuf::from(icon_path);
    let file_name = path_buf.file_name().expect("Failed to get file name");
    let dest_path = icons_path.join(file_name);

    let res = fs::copy(icon_path, &dest_path).expect("Failed to copy file");

    info!("Check file exist result: {:?}", res);
    info!("Cp result: {:?}", res);

    Ok(dest_path.to_string_lossy().to_string())
}
