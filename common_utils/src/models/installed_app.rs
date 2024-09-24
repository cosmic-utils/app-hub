use std::path::PathBuf;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InstalledApp {
    pub name: String,
    pub icon_path: Option<PathBuf>,
    pub app_path: String,
    pub version: Option<String>,
    pub categories: Option<String>,
}
