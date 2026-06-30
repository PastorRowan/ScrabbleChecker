
pub mod service;

use std::{collections::HashMap, path::PathBuf};

type DictHashMap = HashMap<String, Vec<String>>;

pub struct Dictionaries {
    dir: PathBuf,
    hash_map: DictHashMap
}
