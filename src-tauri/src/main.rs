
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

mod paths;

fn main() {

    tauri::Builder::default()
        .setup(|app: &mut tauri::App| {

            let dictionaries_ro_dir = paths::get_dictionaries_ro_dir(app)?;

            let dictionaries_rw_dir = paths::get_dictionaries_rw_dir(app);

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

            }

            Ok(())

        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    scrabblechecker_lib::run()

}
