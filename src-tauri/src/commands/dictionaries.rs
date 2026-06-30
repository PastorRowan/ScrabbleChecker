
use crate::features::dictionaries::Dictionaries;
use std::sync::Mutex;

pub type DictionariesState = Mutex<Dictionaries>;

#[tauri::command]
pub fn get_dictionaries(
    dictionaries: tauri::State<'_, DictionariesState>,
) -> Vec<String> {
    let dictionaries = dictionaries.lock().unwrap();
    return dictionaries.get_dictionaries();
}

#[tauri::command]
pub fn is_word_in_dictionary(
    dictionaries: tauri::State<'_, DictionariesState>,
    dictionary_name: &str,
    word: &str
) -> bool {
    let dictionaries = dictionaries.lock().unwrap();
    return dictionaries.is_word_in_dictionary(&dictionary_name, &word)
}

#[tauri::command]
pub fn create_dictionary(
    dictionaries: tauri::State<'_, DictionariesState>,
    dictionary_name: &str,
    words: &str
) -> Result<(), String> {
    let mut dictionaries = dictionaries.lock().unwrap();
    match dictionaries.create_dictionary(&dictionary_name, &words) {
        Ok(_) => {
            Ok(())
        }
        Err(e) => {
            Err(e.to_string())
        }
    }
}

#[tauri::command]
pub fn delete_dictionary(
    dictionaries: tauri::State<'_, DictionariesState>,
    dictionary_name: &str
) -> Result<(), String> {
    let mut dictionaries = dictionaries.lock().unwrap();
    match dictionaries.delete_dictionary(&dictionary_name) {
        Ok(_) => {
            Ok(())
        }
        Err(e) => {
            Err(e.to_string())
        }
    }
}
