use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use log::{debug, error, info};
use tauri::AppHandle;
use common_utils::desktop_file_builder::DesktopFileBuilder;
use common_utils::desktop_file_helpers::{delete_desktop_file_by_name, find_desktop_entry, find_desktop_file_location};
use common_utils::file_system_helpers::{add_executable_permission, find_desktop_file_in_dir, rm_dir_all, rm_file};
use common_utils::icons_helpers::image_to_base64;
use crate::commands::app_settings_commands::read_settings_command;
use crate::models::app_list::App;
use crate::models::app_settings::AppSettings;
use crate::models::request_installation::RequestInstallation;

/// Install an AppImage
pub async fn install_app_image(app: AppHandle, request_installation: RequestInstallation) -> Result<(), String> {

    let apps_installation_dir_path= match read_settings_command(app).await {
        Ok(settings) => {
            settings.install_path.unwrap()
        }
        Err(err) => {
            error!("{}", err);
            return Err("Failed to read settings".to_string());
        }
    };

    // Print the command args for debugging
    info!("Received install action");
    info!("file_path: {:?}", request_installation.file_path);
    info!("install_dir: {:?}", apps_installation_dir_path);
    info!("no_sandbox: {:?}", request_installation.no_sandbox.unwrap_or(false));

    let path = std::env::current_exe().map_err(|_| "unable to get current exe")?;
    let cmd = Command::new("pkexec")
        .arg(path.parent().unwrap().join("app_hub_backend"))
        .arg("--action")
        .arg("install")
        .arg("--file-path")
        .arg(request_installation.file_path)
        .arg("--install-dir")
        .arg(apps_installation_dir_path)
        .arg("--no-sandbox")
        .arg(request_installation.no_sandbox.unwrap_or(false).to_string())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();

    match cmd {
        Ok(mut child) => {
            // Read stdout
            if let Some(stdout) = child.stdout.take() {
                let reader = BufReader::new(stdout);
                for line in reader.lines() {
                    if let Ok(line) = line {
                        debug!("app_hub_backend output: {}", line);
                    }
                    else {
                        error!("Failed to read line from stdout");
                    }
                }
            }

            // Read stderr
            if let Some(stderr) = child.stderr.take() {
                let reader = BufReader::new(stderr);
                for line in reader.lines() {
                    if let Ok(line) = line {
                        error!("app_hub_backend error: {}", line);
                    }
                    else {
                        error!("Failed to read line from stderr");
                    }
                }
            }

            let output = child.wait_with_output().expect("Failed to wait on child");
            debug!("output: {:?}", output);
            if output.status.success() {
                info!("Installation successful");
            } else {
                error!("Installation failed");
                return Err("Installation failed".to_string());
            }
        }
        Err(error) => {
            error!("error: {:?}", error);
            return Err("Failed to install AppImage".to_string());
        }
    }

    Ok(())
}

/// Read all desktop files in the applications directory
/// This retrieved list contains only files installed by AppHub
pub fn read_all_app() -> Result<Vec<App>, String> {
    let mut apps: Vec<App> = Vec::new();

    // read all .desktop files in the applications directory
    let applications_dir: PathBuf = find_desktop_file_location().map_err(|e| e.to_string())?;

    match std::fs::read_dir(applications_dir) {
        Ok(entries) => {
            for entry in entries {
                let entry = entry.as_ref().unwrap();

                let desktop_file = DesktopFileBuilder::from_desktop_entry_path(&entry.path(), true);

                if desktop_file.is_err() {
                    error!("Failed to read desktop file: {}", desktop_file.err().unwrap());
                    continue;
                }

                let desktop_entry = desktop_file.unwrap();

                let base64_icon: Option<String> = match image_to_base64(desktop_entry.icon().unwrap().as_str()) {
                    Ok(base64) => Some(base64),
                    Err(err) => {
                        debug!("from path: {}", desktop_entry.icon().unwrap());
                        info!("Failed to convert image to base64: {}", err);
                        None
                    }
                };

                apps.push(App {
                    name: desktop_entry.name().unwrap(),
                    icon_base64: base64_icon,
                    app_path: desktop_entry.exec().unwrap(),
                    version: desktop_entry.version(),
                    categories: desktop_entry.categories(),
                });
            }
        }
        Err(err) => {
            return Err(format!("Failed to read directory: {}", err));
        }
    }

    Ok(apps)
}

/// Uninstall an AppImage
pub fn uninstall_app(app_name: String) -> Result<bool, String> {
    let path = std::env::current_exe().map_err(|_| "unable to get current exe")?;
    let cmd = Command::new("pkexec")
        .arg(path.parent().unwrap().join("app_hub_backend"))
        .arg("--action")
        .arg("uninstall")
        .arg("--uninstall-app-name")
        .arg(app_name)
        .spawn();
    match cmd {
        Ok(mut res) => {
            // Leggi lo stdout
            if let Some(stdout) = res.stdout.take() {
                let reader = BufReader::new(stdout);
                for line in reader.lines() {
                    if let Ok(line) = line {
                        debug!("app_hub_backend output: {}", line);
                    }
                }
            }

            // Leggi lo stderr
            if let Some(stderr) = res.stderr.take() {
                let reader = BufReader::new(stderr);
                for line in reader.lines() {
                    if let Ok(line) = line {
                        error!("app_hub_backend error: {}", line); // Stampalo come errore nel logger
                    }
                }
            }

            let output = res.wait_with_output().expect("Failed to wait on child");
            if output.status.success() {
                info!("Uninstallation successful");
            } else {
                error!("Uninstallation failed");
                return Err("Uninstallation failed".to_string());
            }
            Ok(true)
        }
        Err(error) => {
            error!("error: {:?}", error);
            Err("Failed to uninstall AppImage".to_string())
        }
    }
}
