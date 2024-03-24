use std::fs;
use std::os::unix::fs::PermissionsExt;

pub fn install_app_image(file_path: &String) -> Result<String, String> {
    //TODO now move the file to the default directory but in the frontend in the advanced settings
    // the user should be able to choose a custom directory, so this should be a parameter
    // in the request_installation struct and we should use that parameter here to move the file
    // to the custom directory
    let home_dir = dirs::home_dir().expect("Failed to get home directory");
    let app_images_path = home_dir.join("AppImages");

    // Try to create the directory and handle the error if it already exists
    match fs::create_dir(&app_images_path) {
        Ok(_) => {}
        Err(ref e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
            println!("Directory already exists");
        }
        Err(e) => return Err(format!("Failed to create directory: {}", e)),
    }

    let path_buf = std::path::PathBuf::from(file_path);
    let file_name = path_buf.file_name().expect("Failed to get file name");
    let dest_path = app_images_path.join(file_name);

    let res = fs::copy(file_path, &dest_path).expect("Failed to copy file");

    // Set the executable permission to the file
    let mut perms = fs::metadata(&dest_path).expect("Failed to get metadata").permissions();
    // Set the executable permission to the file
    perms.set_mode(0o755);
    fs::set_permissions(&dest_path, perms).expect("Failed to set permissions");

    println!("Check file exist result: {:?}", res);
    println!("Cp result: {:?}", res);

    Ok("Installation successful".to_string())
}

pub fn copy_icon_file(icon_path: &String) -> Result<String, String> {
    let home_dir = dirs::home_dir().expect("Failed to get home directory");
    let icons_path = home_dir.join("AppImages").join("icons");

    // Try to create the directory and handle the error if it already exists
    match fs::create_dir(&icons_path) {
        Ok(_) => {}
        Err(ref e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
            println!("Directory already exists");
        }
        Err(e) => return Err(format!("Failed to create directory: {}", e)),
    }

    let path_buf = std::path::PathBuf::from(icon_path);
    let file_name = path_buf.file_name().expect("Failed to get file name");
    let dest_path = icons_path.join(file_name);

    let res = fs::copy(icon_path, &dest_path).expect("Failed to copy file");

    println!("Check file exist result: {:?}", res);
    println!("Cp result: {:?}", res);

    Ok(dest_path.to_string_lossy().to_string())
}
