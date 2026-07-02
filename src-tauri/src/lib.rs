
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

pub mod commands;
pub mod features;

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(
            |app: &mut tauri::App| {

                const CSW21_WORD_LIST: &[u8] = include_bytes!("../resources/dictionaries/CSW21.txt");
                const CSW24_WORD_LIST: &[u8] = include_bytes!("../resources/dictionaries/CSW24.txt");
                const NSWL2018_WORD_LIST: &[u8] = include_bytes!("../resources/dictionaries/NSWL2018.txt");
                const NSWL2020_WORD_LIST: &[u8] = include_bytes!("../resources/dictionaries/NSWL2020.txt");
                const NSWL2023_WORD_LIST: &[u8] = include_bytes!("../resources/dictionaries/NSWL2023.txt");
                const NWL2018_WORD_LIST: &[u8] = include_bytes!("../resources/dictionaries/NWL2018.txt");
                const NWL2020_WORD_LIST: &[u8] = include_bytes!("../resources/dictionaries/NWL2020.txt");
                const NWL2023_WORD_LIST: &[u8] = include_bytes!("../resources/dictionaries/NWL2023.txt");

                let dictionary_files: [(&str, &[u8]); 8] = [
                    ("CSW21.txt", CSW21_WORD_LIST),
                    ("CSW24.txt", CSW24_WORD_LIST),
                    ("NSWL2018.txt", NSWL2018_WORD_LIST),
                    ("NSWL2020.txt", NSWL2020_WORD_LIST),
                    ("NSWL2023.txt", NSWL2023_WORD_LIST),
                    ("NWL2018.txt", NWL2018_WORD_LIST),
                    ("NWL2020.txt", NWL2020_WORD_LIST),
                    ("NWL2023.txt", NWL2023_WORD_LIST),
                ];

                let dictionaries_rw_dir =
                    app
                    .path()
                    .app_data_dir()
                    .expect("Failed to create dictionaries_rw_dir PathBuf")
                    .join("dictionaries");

                std::fs::create_dir_all(&dictionaries_rw_dir)
                    .expect(
                        &format!("Failed to create_dir_all for dictionaries_rw_dir at {:?}", dictionaries_rw_dir.display())
                    );

                for dictionary_file in dictionary_files {

                    let ( file_name, content ) = dictionary_file;

                    let to = dictionaries_rw_dir.join(file_name);

                    std::fs::write(&to, content)
                        .expect(
                            &format!(
                                "Failed to created dictionary file {:?} at {:?}",
                                file_name,
                                to.display()
                            )
                        );

                    println!(
                        "Successfully created dictionary file {:?} at {:?}",
                        file_name,
                        to.display()
                    );

                };

                let dictionary =
                    features::dictionaries::Dictionaries::new(
                        &dictionaries_rw_dir
                    ).expect("Failed to create dictionary");

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
        .expect("error while running tauri application");

}
