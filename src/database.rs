use std::{
    fs::{File, read_to_string},
    io::Write,
    path::{Path, PathBuf},
};

use crate::constants::{BIN_NAME, DATABASE_NAME};

pub struct Database {
    log_file_path: PathBuf,
}

impl Database {
    pub fn init(xdg_data_home: &Path) -> Self {
        let log_file_path = init_database(xdg_data_home);
        Database { log_file_path }
    }

    pub fn path(&self) -> &Path {
        &self.log_file_path
    }

    pub fn update(&self, new_content: String) {
        let old_content =
            read_to_string(&self.log_file_path).expect("failed to read log file content");

        let content = format_content(new_content, old_content);

        let mut log_file = std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.log_file_path)
            .expect("failed to open log file for appending");

        Write::write_all(&mut log_file, content.as_bytes()).expect("failed to write to log file");
    }
}

fn init_database(xdg_data_home: &Path) -> PathBuf {
    let log_file_path_name = create_log_file_path_name(&xdg_data_home);
    let log_file_path = Path::new(&log_file_path_name);

    if !log_file_path.exists() {
        std::fs::create_dir_all(
            log_file_path
                .parent()
                .expect("LOG.md should have a directory parent"),
        )
        .expect("failed to create directories for log file");
        File::create(&log_file_path).expect("LOG.md should be always created");
    }

    log_file_path.to_path_buf()
}

fn create_log_file_path_name(xdg_data_home: &Path) -> String {
    let xdg_data_home = xdg_data_home
        .to_str()
        .expect("XDG_DATA_HOME should be valid str");
    format!("{xdg_data_home}/{BIN_NAME}/{DATABASE_NAME}",)
}

fn format_content(new_content: String, old_content: String) -> String {
    format!("{new_content}\n{old_content}")
}
