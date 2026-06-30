
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

pub mod commands;
pub mod features;

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(
            |app: &mut tauri::App| {

                let dictionaries_ro_dir =
                    app
                    .path()
                    .resolve("resources/dictionaries", tauri::path::BaseDirectory::Resource)?;

                let dictionaries_rw_dir =
                    app
                    .path()
                    .app_data_dir()
                    .unwrap()
                    .join("dictionaries");

                std::fs::create_dir_all(&dictionaries_rw_dir)?;

                for entry in dictionaries_ro_dir.read_dir()? {
                    let from = entry?.path();
                    let to = dictionaries_rw_dir.join(from.file_name().unwrap());
                    std::fs::copy(&from, &to)?;
                    println!(
                        "Successfully copied {:?} to {:?}",
                        from.display(),
                        to.display()
                    );

                };

                let dictionary =
                    features::dictionaries::Dictionaries::new(&dictionaries_rw_dir)?;

                let dictionaries_state =
                    commands::dictionaries::DictionariesState::new(
                        dictionary
                    );

                app.manage(dictionaries_state);

                Ok(())

            }
        )
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::dictionaries::get_dictionaries,
            commands::dictionaries::is_word_in_dictionary,
            commands::dictionaries::create_dictionary,
            commands::dictionaries::delete_dictionary
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

}
