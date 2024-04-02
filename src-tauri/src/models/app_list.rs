#[derive(serde::Serialize, serde::Deserialize)]
pub struct AppList {
    pub apps: Vec<App>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct App {
    pub name: String,
    pub icon_base64: Option<String>,
    pub app_path: String,
}
