use log::{error, info};
use crate::helpers::file_conversion_helper::image_to_base64;
use crate::models::app_list::App;
use regex::Regex;

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
                // read the .desktop file and get the path of the AppImage
                let file_content: String =
                    std::fs::read_to_string(entry.unwrap().path()).expect("Failed to read file");

                // parse the desktop entry to get the path of the AppImage
                match parse_desktop_entry(&file_content) {
                    Ok((exec, name, icon)) => {
                        //TODO: in this case we are assuming that the AppImage is in /AppImages dir
                        // since it is the default path used by AppHub,
                        // but the user could have installed the AppImage in a different directory
                        // using the advanced settings
                        if exec.contains("/AppImages") {
                            let base64_icon = match image_to_base64(&icon) {
                                Ok(base64) => Some(base64),
                                Err(err) => {
                                    info!("Failed to convert image to base64: {}", err);
                                    None
                                }
                            };

                            apps.push(App {
                                name,
                                icon_base64: base64_icon,
                                app_path: exec,
                            });
                        }
                    }
                    Err(err) => {
                        error!("{}", err)
                    }
                };
            }
        }
        Err(err) => {
            return Err(format!("Failed to read directory: {}", err));
        }
    }

    Ok(apps)
}

// This function is used to parse a desktop entry string and extract the values of "Exec", "Name", and "Icon".
pub fn parse_desktop_entry(desktop_entry: &str) -> Result<(String, String, String), &'static str> {
    // Create a regular expression to match the lines starting with "Exec=", "Name=", and "Icon=".
    let re = Regex::new(r"(?m)^Exec=(.*)$|^Name=(.*)$|^Icon=(.*)$").unwrap();

    // Initialize empty strings to hold the values of "Exec", "Name", and "Icon".
    let mut exec = String::new();
    let mut name = String::new();
    let mut icon = String::new();

    // Iterate over all the matches in the desktop entry string.
    for cap in re.captures_iter(desktop_entry) {
        // If the first capture group (corresponding to "Exec") is matched, store its value.
        if let Some(matched) = cap.get(1) {
            exec = matched.as_str().to_string();
        }
        // If the second capture group (corresponding to "Name") is matched, store its value.
        if let Some(matched) = cap.get(2) {
            name = matched.as_str().to_string();
        }
        // If the third capture group (corresponding to "Icon") is matched, store its value.
        if let Some(matched) = cap.get(3) {
            icon = matched.as_str().to_string();
        }
    }

    // If any of the "Exec", "Name", or "Icon" values are still empty after parsing, return an error.
    if exec.is_empty() || name.is_empty() || icon.is_empty() {
        error!("Failed to parse desktop entry {}", desktop_entry);
        return Err("Failed to parse desktop entry");
    }

    // If all values are successfully extracted, return them as a tuple.
    Ok((exec, name, icon))
}

pub fn find_app_path_by_name(app_name: String) -> Result<String, String> {
    // read all .desktop files in the applications directory
    let home_dir = dirs::home_dir().expect("Failed to get home directory");
    let applications_dir = home_dir.join(".local").join("share").join("applications");
    match std::fs::read_dir(applications_dir) {
        Ok(entries) => {
            for entry in entries {
                // read the .desktop file and get the path of the AppImage
                let file_content: String =
                    std::fs::read_to_string(entry.unwrap().path()).expect("Failed to read file");

                // parse the desktop entry to get the path of the AppImage
                match parse_desktop_entry(&file_content) {
                    Ok((exec, name, _)) => {
                        if name == app_name {
                            return Ok(exec);
                        }
                    }
                    Err(err) => {
                        error!("{}", err)
                    }
                };
            }
            return Err(format!("App not found: {}", app_name));
        }
        Err(err) => {
            return Err(format!("Failed to read directory: {}", err));
        }
    }
}
