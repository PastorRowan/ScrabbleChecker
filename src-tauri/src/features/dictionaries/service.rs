
use serde::de::value::Error;

use super::{
    Dictionaries,
    DictionaryHashMap,
    DictionaryEntry,
    DictionaryEntries
};

impl Dictionaries {

    pub fn new(
        dir: &std::path::PathBuf
    ) -> std::io::Result<Self> {

        let mut dictionary_hash_map = DictionaryHashMap::new();

        for dictionary_file_result in dir.read_dir().expect("Failed to read dir") {
            match dictionary_file_result {
                Ok(file) => {
                    if file.file_type()?.is_file() {
                        let dictionary_name = file.path().file_prefix().unwrap().to_string_lossy().to_string();
                        let words = std::fs::read_to_string(file.path())?;
                        let mut dictionary_entries: DictionaryEntries = Vec::new();
                        for line in words.lines() {
                            match line.split_once(' ') {
                                Some((word, description)) => {
                                    dictionary_entries.push(
                                        DictionaryEntry {
                                            word: word.to_ascii_uppercase(),
                                            description: description.to_string()
                                        }
                                    );
                                }
                                None => {}
                            };
                        };
                        dictionary_entries.sort();
                        dictionary_hash_map.insert(dictionary_name, dictionary_entries);
                    }
                }
                Err(e) => {
                    println!("Error reading dictionary file: {}", e)
                }
            }
        }

        Ok(Self {
            dir: dir.to_path_buf(),
            dictionary_hash_map
        })

    }

}

impl Dictionaries {

    pub fn get_dictionaries(&self) -> Vec<String> {
        let mut dictionary_names: Vec<String> = Vec::with_capacity(self.dictionary_hash_map.len());
        for (dictionary_name, _) in self.dictionary_hash_map.iter() {
            dictionary_names.push(dictionary_name.to_string());
        };
        dictionary_names.sort();
        return dictionary_names
    }

}

impl Dictionaries {

    pub fn lookup_word(
        &self,
        dictionary_name: &str,
        word: &str
    ) -> Option<&DictionaryEntry> {

        if !word.is_ascii() {
            return None;
        };

        let dictionary_entries = self.dictionary_hash_map.get(dictionary_name).unwrap();
        let binary_search_result: Result<usize, usize> = dictionary_entries.binary_search(
            &DictionaryEntry {
                word: word.to_ascii_uppercase(),
                description: "".to_string()
            }
        );
        match binary_search_result {
            Ok(i) => {
                return Some(&dictionary_entries[i])
            }
            Err(_) => {
                return None
            }
        }
    }

}

/*

impl Dictionaries {

    pub fn create_dictionary(
        &mut self,
        dictionary_name: &str,
        words: &str
    ) -> std::io::Result<()> {

        let path = std::path::Path::new(dictionary_name);

        if !dictionary_name.is_ascii() || path.extension().is_some() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid dictionary name"
            ));
        };

        let created_dictionary_path = self.dir.join(dictionary_name);

        std::fs::write(created_dictionary_path, words)?;

        let mut value: Vec<String> = words
            .lines()
            .map(|s| s.to_ascii_lowercase())
            .collect();
        value.sort();

        self.hash_map.insert(dictionary_name.to_string(), value);

        return Ok(())

    }

}

impl Dictionaries {

    pub fn delete_dictionary(
        &mut self,
        dictionary_name: &str
    ) -> std::io::Result<()> {

        let dictionary_path = self.dir.join(dictionary_name);
        if let Err(e) = std::fs::remove_file(&dictionary_path) {
            eprintln!("Failed to delete file '{}'\n{}", dictionary_path.display(), e)
        }

        if let None = self.hash_map.remove(dictionary_name) {
            eprintln!("Failed to remove dictionary '{}' from self.hash_map", dictionary_name)
        }

        Ok(())

    }

}

*/
