
#[tauri::command]
pub fn get_dictionaries() -> String {
    println!("get_dictionaries");
    let test = "get_dictionaries";
    return test.to_string();
}

#[tauri::command]
pub fn is_word_in_dictionary(name: &str, word: &str) -> bool {
    return true
}

#[tauri::command]
pub fn create_dictionary(name: &str, content: &str) -> String {
    println!("create_dictionary name: {name} content: {content}");
    let test = "create_dictionary";
    return test.to_string();
}

#[tauri::command]
pub fn delete_dictionary(name: &str) -> String {
    println!("delete_dictionary name: {name}");
    let test = "delete_dictionary";
    return test.to_string();
}
