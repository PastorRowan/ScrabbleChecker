
use std::fs;
use std::path::Path;

fn main() {

    let dictionary_dir = Path::new("resources/dictionaries");

    println!(
        "Collecting dictionary file names in dictionary directory '{:?}'",
        dictionary_dir
    );

    let mut dictionary_files = Vec::new();

    for entry in fs::read_dir(dictionary_dir).unwrap() {

        let entry = entry.unwrap();

        let path = entry.path();
        let file_ext = path.extension();

        if file_ext.is_none_or(|ext| ext != "txt") {
            println!(
                "dictionary file '{:?}' is not a .txt file, ignoring file",
                path
            );
            continue;
        };

        if !path.is_file() {
            println!(
                "Entry '{:?}' is not a file, ignoring entry",
                dictionary_dir
            );
            continue;
        };

        let Some(file_name) = path.file_name() else {
            println!(
                "Could not determine file name for '{:?}', ignoring file",
                path
            );
            continue;
        };

        let Some(file_name) = file_name.to_str() else {
            println!(
                "File name for '{:?}' is not valid UTF-8, ignoring file",
                path
            );
            continue;
        };

        let file_name_string = file_name.to_string();

        dictionary_files.push(file_name_string.clone());

        println!(
            "Successfully added dictionary file name '{:?}' from '{:?}' to dictionary_files",
            &file_name_string,
            &path
        );

    };

    dictionary_files.sort();

    let mut generated_code = String::new();

    generated_code.push_str("pub const DICTIONARY_FILES: &[&str] = &[\n");

    for file_name in &dictionary_files {
        generated_code.push_str(&format!("    {:?},\n", file_name));
    };

    generated_code.push_str("];\n");

    let out_dir = std::env::var("OUT_DIR").unwrap();

    let destination = Path::new(&out_dir).join("dictionary_files.rs");

    fs::write(destination, generated_code).unwrap();

    tauri_build::build();

}
