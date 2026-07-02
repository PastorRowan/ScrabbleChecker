
use crate::features::dictionaries::Dictionaries;
use crate::features::dictionaries::DictionaryEntry;

use std::sync::Mutex;

pub type DictionariesState = Mutex<Dictionaries>;

use serde::Serialize;

#[derive(Serialize)]
pub struct DictionariesCommandResponse<T = ()> {
    pub ok: bool,
    pub error_msg: Option<String>,
    pub result: Option<T>,
}

#[tauri::command]
pub fn get_dictionaries(
    dictionaries: tauri::State<'_, DictionariesState>,
) -> DictionariesCommandResponse<Vec<String>> {
    let dictionaries = dictionaries.lock().unwrap();
    let result = dictionaries.get_dictionaries();
    return DictionariesCommandResponse {
        ok: true,
        error_msg: None,
        result: Some(result)
    }
}

#[tauri::command]
pub fn lookup_word(
    dictionaries: tauri::State<'_, DictionariesState>,
    dictionary_name: &str,
    word: &str
) -> DictionariesCommandResponse<DictionaryEntry> {
    println!("lookup_word");
    println!("dictionary_name: {}", dictionary_name);
    println!("word: {}", word);
    let dictionaries = dictionaries.lock().unwrap();
    match dictionaries.lookup_word(&dictionary_name, &word) {
        Some(dictionary_entry) => {
            return DictionariesCommandResponse {
                ok: true,
                error_msg: None,
                result: Some(dictionary_entry.clone())
            }
        }
        None => {
            return DictionariesCommandResponse {
                ok: true,
                error_msg: None,
                result: None
            }
        }
    }

}

/*
#[tauri::command]
pub fn create_dictionary(
    dictionaries: tauri::State<'_, DictionariesState>,
    dictionary_name: &str,
    words: &str
) -> DictionariesCommandResponse {
    let mut dictionaries = dictionaries.lock().unwrap();
    match dictionaries.create_dictionary(&dictionary_name, &words) {
        Ok(_) => {
            return DictionariesCommandResponse {
                ok: true,
                error_msg: None,
                result: None
            }
        }
        Err(_) => {
            return DictionariesCommandResponse {
                ok: false,
                error_msg: Some("Failed to create dictionary".to_string()),
                result: None
            }
        }
    }
}

#[tauri::command]
pub fn delete_dictionary(
    dictionaries: tauri::State<'_, DictionariesState>,
    dictionary_name: &str
) -> DictionariesCommandResponse {
    let mut dictionaries = dictionaries.lock().unwrap();
    match dictionaries.delete_dictionary(&dictionary_name) {
        Ok(_) => {
            return DictionariesCommandResponse {
                ok: true,
                error_msg: None,
                result: None
            }
        }
        Err(_) => {
            return DictionariesCommandResponse {
                ok: false,
                error_msg: Some("Failed to delete dictionary".to_string()),
                result: None
            }
        }
    }
}

*/
