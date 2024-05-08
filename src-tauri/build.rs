fn main() {
    // set env var to enable tauri to build the app
    std::env::set_var("NO_STRIP", "true");
    tauri_build::build()
}
