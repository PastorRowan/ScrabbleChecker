
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {

    use tauri::Manager;

    tauri::Builder::default()
        .setup(|app| {
            let readonly_dictionaries_dir = app
                .path()
                .resolve("resources/dictionaries", tauri::path::BaseDirectory::Resource)?;

            let rw_dictionaries_dir: std::path::PathBuf = app
                .path()
                .app_data_dir()?
                .join("dictionaries");

            for entry in readonly_dictionaries_dir
                .read_dir()
                .expect("Failed to read dictionaries_dir") {

                match entry {
                    Ok(entry) => {
                        let from = entry.path();
                        match from.file_name() {
                            Some(file_n) => {
                                let to = rw_dictionaries_dir.join(file_n);
                                std::fs::copy(&from, &to)?;
                                // file_n.to_str()?
                                match file_n.to_str() {
                                    Some(file_n_str) => {
                                        let to_file_name_str = to.to_str().unwrap();
                                        println!("Successfully copied dictionary from {file_n_str} to {to_file_name_str}");
                                    }
                                    None => {
                                        println!("Failed to convert file name to str");
                                    }
                                }
                            }
                            None => {
                                println!("Path has no filename");
                            }
                        }
                    }
                    Err(err) => {
                        println!("Failed to read directory: {err}")
                    }
                }

            }

            Ok(())

        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    scrabblechecker_lib::run()

}
