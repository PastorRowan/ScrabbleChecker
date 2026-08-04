
pub mod service;

use std::{collections::HashMap, path::PathBuf};

use std::cmp::Ordering;

use serde::Serialize;

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub struct DictionaryEntry {
    word: String,
    description: String
}

impl Ord for DictionaryEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.word.cmp(&other.word)
    }
}

impl PartialOrd for DictionaryEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other))
    }
}

pub type DictionaryEntries = Vec<DictionaryEntry>;

pub type DictionaryHashMap = HashMap<String, Vec<DictionaryEntry>>;

pub struct Dictionaries {
    dir: PathBuf,
    dictionary_hash_map: DictionaryHashMap
}
