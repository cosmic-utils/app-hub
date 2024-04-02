use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    pub theme: String,
    pub language: String,
    pub install_path: Option<String>, // Path where the apps are installed
    pub create_desktop_entry: bool,
}
