
use std::{collections::HashMap};

pub struct Dictionaries {
    dir: std::path::PathBuf,
    hash_map: HashMap<String, Vec<String>>
}

impl Dictionaries {
    pub fn new(
        dir: std::path::PathBuf
    ) -> Self {
        Self {
            dir: dir,
            hash_map: HashMap::new()
        }
    }
}

/*
impl Dictionaries {
    fn split_lines(
        content: &String
    ) -> Vec<&str> {
        let v: Vec<&str> = content.split('\n').collect();
        return v;
    }
}
*/

impl Dictionaries {
    pub fn get_dictionaries(&self) -> Vec<String> {
        let mut dictionary_names: Vec<String> = Vec::with_capacity(self.hash_map.len());
        for (dictionary_name, _) in self.hash_map.iter() {
            dictionary_names.push(dictionary_name.to_string());
        };
        dictionary_names.sort();
        return dictionary_names
    }
}

impl Dictionaries {
    pub fn is_word_in_dictionary(&self, dictionary_name: &str, word: &String) -> bool {
        let dictionary_words: &Vec<String> = self.hash_map.get(dictionary_name).unwrap();
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
    pub fn create_dictionary(&mut self, dictionary_name: &String, content: &String) -> String {
        let value: Vec<String> = content
            .lines()
            .map(|s| s.to_string())
            .collect();
        self.hash_map.insert(dictionary_name.to_string(), value);
        let created_dictionary_path = self.dir.join(dictionary_name);
        std::fs::write(created_dictionary_path, contents);
        return "create_dictionary called".to_string();
    }
}

impl Dictionaries {
    pub fn delete_dictionary(dictionary_name: &str) -> str {
        println!("delete_dictionary name: {dictionary_name}");
        let test = "delete_dictionary";
        return test.to_string();
    }
}
