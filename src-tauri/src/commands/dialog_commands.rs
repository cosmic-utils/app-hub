use tauri::api::dialog::blocking::FileDialogBuilder;

#[tauri::command]
pub async fn choose_file() -> Result<String, String> {
    let dialog_result = FileDialogBuilder::new().pick_file();

    match dialog_result {
        Some(path) => {
            println!("Selected file: {}", path.to_string_lossy());
            Ok(path.into_os_string().into_string().unwrap_or_else(|_| "Invalid path".to_string()))
        },
        None => Err("No file selected".to_string())
    }
}
