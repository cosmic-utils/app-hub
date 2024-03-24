use log::info;
use crate::models::request_installation;
use crate::models::request_installation::RequestInstallation;

#[tauri::command]
pub async fn install_file(request_installation: RequestInstallation) -> Result<String, String> {

    info!("Installing file: {:?}", request_installation);

    Ok("Installation successful".to_string())

}
