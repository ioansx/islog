use std::{
    fs::{File, read_to_string},
    io::Write,
    path::{Path, PathBuf},
};

use crate::{
    constants::{BIN_NAME, DATABASE_NAME},
    error::{Errx, Resultx},
};

pub struct Database {
    log_file_path: PathBuf,
}

impl Database {
    pub fn init(xdg_data_home: &Path) -> Resultx<Self> {
        let log_file_path = init_database(xdg_data_home)?;
        Ok(Database { log_file_path })
    }

    pub fn path(&self) -> &Path {
        &self.log_file_path
    }

    pub fn update(&self, new_content: String) -> Resultx<()> {
        let old_content = read_to_string(&self.log_file_path)
            .map_err(|e| Errx::e_io(e, format!("reading existing content from {DATABASE_NAME}")))?;

        let content = format_content(new_content, old_content);

        let mut log_file = std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.log_file_path)
            .map_err(|e| Errx::e_io(e, format!("opening {DATABASE_NAME} for writing")))?;

        Write::write_all(&mut log_file, content.as_bytes())
            .map_err(|e| Errx::e_io(e, format!("writing to {DATABASE_NAME}")))
    }
}

fn init_database(xdg_data_home: &Path) -> Resultx<PathBuf> {
    let log_file_path_name = create_log_file_path_name(xdg_data_home)?;
    let log_file_path = Path::new(&log_file_path_name);

    if !log_file_path.exists() {
        std::fs::create_dir_all(log_file_path.parent().ok_or_else(|| -> Errx {
            Errx::io(format!("{DATABASE_NAME} should have a directory parent"))
        })?)
        .map_err(|e| {
            Errx::e_io(
                e,
                format!("creating directories for log file at {:?}", log_file_path),
            )
        })?;
        File::create(log_file_path).expect("{DATABASE_NAME} should be always created");
    }

    Ok(log_file_path.to_path_buf())
}

fn create_log_file_path_name(xdg_data_home: &Path) -> Resultx<String> {
    let xdg_data_home = xdg_data_home
        .to_str()
        .ok_or_else(|| Errx::io("XDG_DATA_HOME is not valid str"))?;
    Ok(format!("{xdg_data_home}/{BIN_NAME}/{DATABASE_NAME}"))
}

fn format_content(new_content: String, old_content: String) -> String {
    format!("{new_content}\n{old_content}")
}
