use std::path::Path;

use crate::constants::{BIN_NAME, DATABASE_NAME};

pub fn create_log_file_path_name(xdg_data_home: &Path) -> String {
    let xdg_data_home = xdg_data_home
        .to_str()
        .expect("XDG_DATA_HOME should be valid str");
    format!("{xdg_data_home}/{BIN_NAME}/{DATABASE_NAME}",)
}
