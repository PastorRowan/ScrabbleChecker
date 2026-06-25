
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

/*
#[tauri::command]
fn get_scrabble_dictionary_file_names() -> Vec<String> {
    let resource_path = app.path().resolve("lang/de.json", BaseDirectory::Resource)?;
    return Vec<String>()
}

#[tauri::command]
fn delete_scrabble_dictionary_file() -> String {

}

#[tauri::command]
fn create_scrabble_dictionary_file(name: &str, words: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
*/

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
