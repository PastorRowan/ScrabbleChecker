
pub struct Env {
    pub dictionaries_ro_dir: std::path::PathBuf,
    pub dictionaries_rw_dir: std::path::PathBuf,
}

impl Env {
    pub fn new(
        dictionaries_ro_dir: std::path::PathBuf,
        dictionaries_rw_dir: std::path::PathBuf
    ) -> Env {
        Self {
            dictionaries_ro_dir,
            dictionaries_rw_dir
        }
    }
}
