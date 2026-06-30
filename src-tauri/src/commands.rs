
use crate::env::Env;
use crate::dictionaries::Dictionaries;
use std::sync::Mutex;

#[tauri::command]
pub fn get_dictionaries(
    dictionaries: tauri::State<'_, Mutex<Dictionaries>>,
) -> Vec<String> {
    let dictionaries = dictionaries.lock().unwrap();
    return dictionaries.get_dictionaries();
}

#[tauri::command]
pub fn is_word_in_dictionary(
    dictionaries: tauri::State<'_, Mutex<Dictionaries>>,
    name: String,
    word: String
) -> bool {
    let dictionaries = dictionaries.lock().unwrap();
    return dictionaries.is_word_in_dictionary(&name, &word)
}

#[tauri::command]
pub fn create_dictionary(
    dictionaries: tauri::State<'_, Mutex<Dictionaries>>,
    env: tauri::State<'_, Env>,
    name: String,
    content: String
) -> Result<(), String> {
    let mut dictionaries = dictionaries.lock().unwrap();
    dictionaries.create_dictionary(&env, &name, &content)
}

#[tauri::command]
pub fn delete_dictionary(name: &str) -> String {
    println!("delete_dictionary name: {name}");
    let test = "delete_dictionary";
    return test.to_string();
}
