use dirs;
use log::{error, info};

use crate::helpers::app_images_helpers::{copy_icon_file, install_app_image, rm_file};
use crate::helpers::desktop_file_creator::DesktopFileBuilder;
use crate::helpers::desktop_file_helpers::{find_app_path_by_name, read_all_app};
use crate::models::app_list::{App, AppList};
use crate::models::request_installation::RequestInstallation;

#[tauri::command]
pub async fn install_app(request_installation: RequestInstallation) -> Result<String, String> {
    println!("Installing file: {:?}", request_installation);

    println!("### REQUESTED TO INSTALL APP ###");
    println!("File path: {:?}", request_installation.file_path);
    println!("Icon path: {:?}", request_installation.icon_path);
    println!("App name: {:?}", request_installation.app_name);
    println!(
        "App description: {:?}",
        request_installation.app_description
    );
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
    } else {
        desktop_builder.set_type("Application".to_string());
    }
    desktop_builder.set_version("1.0".to_string()); //TODO: make this settable by the user as advanced setting
    desktop_builder.set_name(request_installation.app_name.clone());
    desktop_builder.set_exec(format!(
        "{}/AppImages/{}",
        dirs::home_dir().unwrap().to_string_lossy(),
        file_name.to_string_lossy()
    ));

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
    let desktop_entry_path = home_dir
        .join(".local")
        .join("share")
        .join("applications")
        .join(format!("{}.desktop", request_installation.app_name));

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

#[tauri::command]
pub async fn read_app_list() -> Result<AppList, String> {
    let apps: Vec<App> = read_all_app()?;
    Ok(AppList { apps })
}

#[tauri::command]
pub async fn uninstall_app(app: App) -> Result<bool, String> {
    let app_path = match find_app_path_by_name(app.name.clone()) {
        Ok(path) => path,
        Err(err) => {
            return Err(err);
        }
    };

    info!("Uninstalling app at: {:?}", app_path);

    // Remove the AppImage
    let app_removed: bool = match rm_file(app_path) {
        Ok(result) => {
            info!("AppImage removed successfully");
            result
        }
        Err(err) => {
            error!("{}", err);
            return Err(err);
        }
    };

    // Remove the icon



    Ok(true)
}
