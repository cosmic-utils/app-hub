use log::{debug, error, info};
use crate::helpers::file_system_helper::image_to_base64;
use crate::models::app_list::App;
use regex::Regex;
use crate::helpers::desktop_file_builder::DesktopFileBuilder;
use crate::models::desktop_entry::DesktopEntry;

/// Read all desktop files in the applications directory
/// This retrieved list contains only files installed by AppHub
pub fn read_all_app() -> Result<Vec<App>, String> {
    let mut apps: Vec<App> = Vec::new();

    // read all .desktop files in the applications directory
    let home_dir = dirs::home_dir().expect("Failed to get home directory");
    let applications_dir = home_dir.join(".local").join("share").join("applications");
    match std::fs::read_dir(applications_dir) {
        Ok(entries) => {
            for entry in entries {
                let entry = entry.as_ref().unwrap();

                let desktop_file = DesktopFileBuilder::from_desktop_entry_path(entry.path().to_str().unwrap().to_string());

                if desktop_file.is_err() {
                    error!("Failed to read desktop file: {}", desktop_file.err().unwrap());
                    continue;
                }

                let desktop_entry = desktop_file.unwrap();

                debug!("Reading icon at: {:?}", desktop_entry.icon());
                let base64_icon = match image_to_base64(&desktop_entry.icon().unwrap()) {
                    Ok(base64) => Some(base64),
                    Err(err) => {
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

/// Find the desktop entry of the application with the given name.
/// The function reads all the .desktop files in the applications directory and compares the "Name" value
/// of each file with the given app_name. If a match is found, the function returns the DesktopEntry struct
pub fn find_desktop_entry(app_name: String) -> Result<DesktopEntry, String> {
    // read all .desktop files in the applications directory
    let home_dir = dirs::home_dir().expect("Failed to get home directory");
    let applications_dir = home_dir.join(".local").join("share").join("applications");
    match std::fs::read_dir(applications_dir) {
        Ok(entries) => {
            for entry in entries {
                let entry = match entry {
                    Ok(entry) => {
                        entry
                    }
                    Err(error) => {
                        error!("{}", error);
                        continue; // Skip the current entry
                    }
                };


                let entry_path = entry.path();
                let path_str = entry_path.to_str().ok_or("Failed to convert path to string")?;
                let desktop_file = DesktopFileBuilder::from_desktop_entry_path(path_str.to_string());

                match desktop_file {
                    Ok(desktop_entry) => {
                        if desktop_entry.name().unwrap() == app_name {
                            return Ok(DesktopEntry {
                                exec: desktop_entry.exec().unwrap(),
                                name: desktop_entry.name().unwrap(),
                                icon_path: desktop_entry.icon().unwrap(),
                            });
                        }
                    }
                    Err(err) => {
                        error!("Failed to read desktop file: {}", err);
                        continue;
                    }
                }
            }
            return Err(format!("App not found: {}", app_name));
        }
        Err(err) => {
            return Err(format!("Failed to read directory: {}", err));
        }
    }
}

/// Find the desktop entries containing the given string in the "Exec" value.
/// The function reads all the .desktop files in the applications directory and compares the "Exec" value
/// of each file with the given contains_exec. If a match is found, the function returns a vector of paths
/// to the .desktop files.
pub fn find_desktop_entries_by_exec_contains(contains_exec: &String) -> Result<Vec<String>, String> {
    let mut desktop_entries_paths: Vec<String> = Vec::new();

    // read all .desktop files in the applications directory
    let home_dir = dirs::home_dir().expect("Failed to get home directory");
    let applications_dir = home_dir.join(".local").join("share").join("applications");
    match std::fs::read_dir(applications_dir) {
        Ok(entries) => {
            for entry in entries {
                let entry = match entry {
                    Ok(entry) => {
                        entry
                    }
                    Err(error) => {
                        error!("{}", error);
                        continue; // Skip the current entry
                    }
                };


                let entry_path = entry.path();
                let path_str = entry_path.to_str().ok_or("Failed to convert path to string")?;
                let desktop_file = DesktopFileBuilder::from_desktop_entry_path(path_str.to_string());

                match desktop_file {
                    Ok(desktop_entry) => {
                        if desktop_entry.exec().unwrap().contains(contains_exec) {
                            desktop_entries_paths.push(path_str.to_string());
                        }
                    }
                    Err(error) => {
                        error!("{}", error);
                    }
                }
            }
        }
        Err(err) => {
            return Err(format!("Failed to read directory: {}", err));
        }
    }

    Ok(desktop_entries_paths)
}

/// Delete the desktop file of the application with the given name.
/// The function returns true if the file is successfully deleted, and false otherwise.
/// If the file is not found, the function returns an error message.
/// The function reads all the .desktop files in the applications directory and compares the "Name" value
/// of each file with the given app_name. If a match is found, the file is deleted.
pub fn delete_desktop_file_by_name(app_name: &String) -> Result<bool, String> {
    let home_dir = dirs::home_dir().expect("Failed to get home directory");
    let applications_dir = home_dir.join(".local").join("share").join("applications");

    match std::fs::read_dir(applications_dir) {
        Ok(entries) => {
            for entry in entries {

                let entry = match entry {
                    Ok(entry) => {
                        entry
                    }
                    Err(error) => {
                        error!("{}", error);
                        continue; // Skip the current entry
                    }
                };


                let entry_path = entry.path();
                let path_str = entry_path.to_str().ok_or("Failed to convert path to string")?;
                let desktop_file = DesktopFileBuilder::from_desktop_entry_path(path_str.to_string());

                match desktop_file {
                    Ok(desktop_entry) => {
                        match desktop_entry.name() {
                            Some(name) => {
                                if name == *app_name {
                                    match std::fs::remove_file(path_str) {
                                        Ok(_) => {
                                            return Ok(true);
                                        }
                                        Err(err) => {
                                            return Err(format!("Failed to remove file: {}", err));
                                        }
                                    }
                                }
                            }
                            None => {
                                error!("Failed to read desktop file: Name is None");
                                continue;
                            }
                        }
                    }
                    Err(error) => {
                        error!("{}", error);
                    }
                }
            }
            return Err(format!("App not found: {}", app_name));
        }
        Err(err) => {
            return Err(format!("Failed to read directory: {}", err));
        }
    }
}
