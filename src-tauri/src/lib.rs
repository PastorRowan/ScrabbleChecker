
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use tauri::Manager;

mod paths;

#[tauri::command]
fn get_dictionaries() -> String {
    println!("get_dictionaries");
    let test = "asdasd";
    return test.to_string();
}

fn is_word_in_dictionary(name: &str, word: &str) -> bool {
    return true
}

#[tauri::command]
fn create_dictionary(name: &str, content: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn delete_dictionary(name: &str) {

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
