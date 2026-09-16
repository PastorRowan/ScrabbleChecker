
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

pub mod commands;
pub mod features;

use std::path::PathBuf;

use tauri::Manager;
use tauri_plugin_fs::FsExt;

include!(concat!(env!("OUT_DIR"), "/dictionary_files.rs"));

pub fn get_dictionary_files_dir(app: &mut tauri::App) -> PathBuf {
    return app.path()
        .resource_dir()
        .expect("Failed to get resources directory")
        .join("dictionaries");
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(
            |app: &mut tauri::App| {

                let dictionaries_rw_dir: PathBuf =
                    app
                    .path()
                    .app_data_dir()
                    .expect("Failed to create dictionaries_rw_dir PathBuf")
                    .join("dictionaries");

                std::fs::create_dir_all(&dictionaries_rw_dir)
                    .expect(
                        &format!(
                            "Failed to create_dir_all for dictionaries_rw_dir at '{:?}'",
                            dictionaries_rw_dir
                        )
                    );

                for file_name in DICTIONARY_FILES  {

                    let dictionary_source_file_path =
                        get_dictionary_files_dir(app)
                        .join(file_name);

                    let contents = match app.fs().read_to_string(&dictionary_source_file_path) {
                        Ok(contents) => contents,
                        Err(error) => {
                            println!(
                                "Failed to extract dictionary contents at '{:?}' error: {:?}",
                                dictionary_source_file_path,
                                error
                            );
                            continue;
                        }
                    };

                    let dictionary_destination_file_path = dictionaries_rw_dir.join(file_name);

                    std::fs::write(
                        &dictionary_destination_file_path,
                        &contents
                    ).expect(
                        &format!(
                            "Failed to write dictionary from '{:?}' to '{:?}'",
                            dictionary_source_file_path,
                            dictionary_destination_file_path
                        )
                    );

                };

                let dictionary =
                    features::dictionaries::Dictionaries::new(
                        &dictionaries_rw_dir
                    ).expect(
                        &format!(
                            "Failed to create dictionary with directory '{:?}'",
                            dictionaries_rw_dir
                        )
                    );

                let dictionaries_state =
                    commands::dictionaries::DictionariesState::new(
                        dictionary
                    );

                app.manage(dictionaries_state);

                return Ok(());

            }

        )
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::dictionaries::get_dictionaries,
            commands::dictionaries::lookup_word,
        ])
        .run(tauri::generate_context!())
        .expect("Failed to build tauri application");

}
