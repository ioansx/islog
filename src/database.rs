use std::{
    fs::{File, read_to_string},
    io::Write,
    path::{Path, PathBuf},
};

use crate::log_file::create_log_file_path_name;

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
        let log_file_content =
            read_to_string(&self.log_file_path).expect("failed to read log file content");

        let mut log_file = std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.log_file_path)
            .expect("failed to open log file for appending");

        Write::write_all(&mut log_file, new_content.as_bytes())
            .expect("failed to write to log file");
        Write::write_all(&mut log_file, b"\n").expect("failed to write newline to log file");
        Write::write_all(&mut log_file, log_file_content.as_bytes())
            .expect("failed to append old log file content");
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
