use std::fs;
use crate::models::request_installation::RequestInstallation;
use dirs;
use crate::models::desktop_entry::DesktopEntry;

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

    let desktop_entry = DesktopEntry::new(
        "Application".to_string(),
        "1.0".to_string(),
        request_installation.app_name.unwrap_or(file_name.to_string_lossy().to_string()),
        request_installation.app_description.unwrap_or("".to_string()),
        "/".to_string(),
        format!("{}/AppImages/{}", dirs::home_dir().unwrap().to_string_lossy(), file_name.to_string_lossy()),
        icon_path,
        request_installation.terminal.unwrap_or(false),
        vec!["Utility".to_string()],
    );

    match create_desktop_entry(desktop_entry) {
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

fn create_desktop_entry(desktop_entry: DesktopEntry) -> Result<String, String> {
    let home_dir = dirs::home_dir().expect("Failed to get home directory");

    let desktop_entry_path = home_dir.join(".local").join("share").join("applications").join("desktop_entry.desktop");

    let content = format!(
        "[Desktop Entry]\n\
        Type={}\n\
        Version={}\n\
        Name={}\n\
        Comment={}\n\
        Path={}\n\
        Exec={}\n\
        Icon={}\n\
        Terminal={}\n\
        Categories={}",
        desktop_entry.type_,
        desktop_entry.version,
        desktop_entry.name,
        desktop_entry.comment,
        desktop_entry.path,
        desktop_entry.exec,
        desktop_entry.icon,
        desktop_entry.terminal,
        desktop_entry.categories
    );

    match fs::write(desktop_entry_path, content) {
        Ok(_) => {
            println!("Desktop entry created successfully");
        }
        Err(_) => {
            return Err("Failed to create desktop entry".to_string());
        }
    }

    Ok("Installation successful".to_string())
}
