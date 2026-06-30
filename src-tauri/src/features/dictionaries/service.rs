
use super::Dictionaries;
use super::DictHashMap;

impl Dictionaries {

    pub fn new(
        dir: &std::path::PathBuf
    ) -> std::io::Result<Self> {

        let mut hash_map = DictHashMap::new();

        for dictionary_file_result in dir.read_dir()? {
            match dictionary_file_result {
                Ok(file) => {

                    if file.file_type()?.is_file() {
                        let dictionary_name = file.file_name()
                            .to_string_lossy()
                            .to_string();
                        let words = std::fs::read_to_string(file.path())?;
                        let value = words
                            .lines()
                            .map(|s| s.to_string())
                            .collect();
                        hash_map.insert(dictionary_name.to_string(), value);
                    }

                }
                Err(e) => {
                    println!("Error reading dictionary file: {}", e)
                }
            }
        }

        Ok(Self {
            dir: dir.to_path_buf(),
            hash_map: hash_map
        })

    }

    pub fn get_dictionaries(&self) -> Vec<String> {
        let mut dictionary_names: Vec<String> = Vec::with_capacity(self.hash_map.len());
        for (dictionary_name, _) in self.hash_map.iter() {
            dictionary_names.push(dictionary_name.to_string());
        };
        dictionary_names.sort();
        return dictionary_names
    }

    pub fn is_word_in_dictionary(
        &self,
        dictionary_name: &str,
        word: &str
    ) -> bool {
        let dictionary_words: &Vec<String> = self.hash_map.get(dictionary_name).unwrap();
        let index: Result<usize, usize> = dictionary_words.binary_search(&word.to_string());
        match index {
            Ok(_) => {
                return true
            }
            Err(_) => {
                return false
            }
        }
    }

    pub fn create_dictionary(
        &mut self,
        dictionary_name: &str,
        words: &str
    ) -> std::io::Result<()> {

        let created_dictionary_path = self.dir.join(dictionary_name);
        std::fs::write(created_dictionary_path, words)?;

        let value: Vec<String> = words
            .lines()
            .map(|s| s.to_string())
            .collect();

        self.hash_map.insert(dictionary_name.to_string(), value);

        Ok(())

    }

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
