
use std::collections::HashMap;

pub struct Dictionaries {
    hash_map: HashMap<String, Vec<String>>
}

impl Dictionaries {
    pub fn new() -> Self {
        Self {
            hash_map: HashMap::new()
        }
    }
}

impl Dictionaries {
    fn get_dictionaries(&self) -> Vec<String> {
        let mut dictionary_names: Vec<String> = Vec::with_capacity(self.hash_map.len());
        for (dictionary_name, _) in self.hash_map.iter() {
            dictionary_names.push(dictionary_name.to_string());
        };
        dictionary_names.sort();
        return dictionary_names
    }
}

impl Dictionaries {
    fn is_word_in_dictionary(&self, dictionary_name: &String, word: &String) -> bool {
        let dictionary_words = self.hash_map.get(dictionary_name).unwrap();
        let index = dictionary_words.binary_search(word);
        match index {
            Ok(_) => {
                return true
            }
            Err(_) => {
                return false
            }
        }
    }
}

impl Dictionaries {
    fn create_dictionary(mut &self, dictionary_name: &String, content: &String) -> String {
        println!("create_dictionary name: {name} content: {content}");
        let test = "create_dictionary";
        return test.to_string();
    }
}

impl Dictionaries {
    fn delete_dictionary(dictionary_name: &String) -> String {
        println!("delete_dictionary name: {name}");
        let test = "delete_dictionary";
        return test.to_string();
    }
}
