
use tauri::Manager;

pub fn get_dictionaries_ro_dir(app: &tauri::App) -> Result<std::path::PathBuf, tauri::Error> {
    return app.path().resolve("resources/dictionaries", tauri::path::BaseDirectory::Resource);
}

pub fn get_dictionaries_rw_dir(app: &tauri::App) -> std::path::PathBuf {
    return app.path().app_data_dir().unwrap().join("dictionaries");
}
