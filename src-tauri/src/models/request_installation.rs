use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestInstallation {
    pub file_path: String,
    pub icon_path: String,
    pub app_name: String,
    pub app_description: String,
    pub app_type: String,
    pub terminal: bool,
}