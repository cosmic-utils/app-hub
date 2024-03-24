use std::fs;
use std::os::unix::fs::PermissionsExt;
use crate::models::request_installation::RequestInstallation;
use dirs;
use crate::helpers::desktop_file_creator::DesktopFileBuilder;

#[tauri::command]
pub async fn install_app(request_installation: RequestInstallation) -> Result<String, String> {
    println!("Installing file: {:?}", request_installation);

    println!("### REQUESTED TO INSTALL APP ###");
    println!("File path: {:?}", request_installation.file_path);
    println!("Icon path: {:?}", request_installation.icon_path);
    println!("App name: {:?}", request_installation.app_name);
    println!("App description: {:?}", request_installation.app_description);
    println!("App type: {:?}", request_installation.app_type);
    println!("Terminal: {:?}", request_installation.terminal);
    println!("#################################");

    match install_app_image(&request_installation.file_path) {
        Ok(_) => {
            println!("AppImage installation successful");
        }
        Err(err) => {
            return Err(err);
        }
    }

    let path_buf = std::path::PathBuf::from(&request_installation.file_path);
    let file_name = path_buf.file_name().expect("Failed to get file name");

    let icon_path = match copy_icon_file(&request_installation.icon_path) {
        Ok(path) => {
            println!("Icon file copied to: {:?}", path);
            path
        }
        Err(err) => {
            return Err(err);
        }
    };

    let mut desktop_builder = DesktopFileBuilder::new();

    // Set mandatory fields
    if request_installation.app_type.is_some() {
        desktop_builder.set_type(request_installation.app_type.unwrap());
    }
    else {
        desktop_builder.set_type("Application".to_string());
    }
    desktop_builder.set_version("1.0".to_string()); //TODO: make this settable by the user as advanced setting
    desktop_builder.set_name(request_installation.app_name);
    desktop_builder.set_exec(format!("{}/AppImages/{}", dirs::home_dir().unwrap().to_string_lossy(), file_name.to_string_lossy()));

    // Set optional fields
    //TODO check params from frontend and set them here
    desktop_builder.set_icon(icon_path);
    desktop_builder.set_terminal(request_installation.terminal.unwrap_or(false));
    //desktop_builder.set_categories("Utility".to_string());
    if request_installation.app_description.is_some() {
        desktop_builder.set_comment(request_installation.app_description.unwrap());
    }

    // Create destingation path
    let home_dir = dirs::home_dir().expect("Failed to get home directory");
    let desktop_entry_path = home_dir.join(".local").join("share").join("applications").join("desktop_entry.desktop");

    // Set no sandbox
    if request_installation.no_sandbox.is_some() && request_installation.no_sandbox.unwrap() {
        println!("Setting no sandbox");
        desktop_builder.set_no_sandbox(true);
    }

    // Create the desktop entry
    match desktop_builder.write_to_file(desktop_entry_path.to_string_lossy().to_string()) {
        Ok(_) => {
            println!("Desktop entry created successfully");
        }
        Err(err) => {
            return Err(err);
        }
    }

    Ok("Installation successful".to_string())
}


fn install_app_image(file_path: &String) -> Result<String, String> {
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

fn copy_icon_file(icon_path: &String) -> Result<String, String> {
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
