use crate::constants::{BIN_NAME, DATABASE_NAME};

pub fn create_log_file_path_name(xdg_data_home: &str) -> String {
    format!("{xdg_data_home}/{BIN_NAME}/{DATABASE_NAME}")
}
