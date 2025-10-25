use std::{
    fs::{File, read_to_string},
    io::Write,
    path::{Path, PathBuf},
};

use crate::{
    constants::{BIN_NAME, DATABASE_NAME, TITLE},
    error::{Errx, Resultx},
    sanitizer, time, validator,
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
        let new_content_lines = new_content.lines().map(|x| x.to_owned()).collect();

        validator::validate_new_content(&new_content)?;
        let new_content_lines = sanitizer::sanitize_new_content(new_content_lines);

        let old_content = read_old_content(&self.log_file_path)?;
        let old_content_lines = old_content.lines().map(|x| x.to_owned()).collect();

        let content = format_content(new_content_lines, old_content_lines);

        let mut log_file = std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.log_file_path)
            .map_err(|e| Errx::e_io(e, format!("opening {DATABASE_NAME} for writing")))?;

        write_to_database(&mut log_file, &content)
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

    let old_content = read_old_content(log_file_path)?;

    // Ensure the database has a title and today's subtitle.
    if old_content.trim().is_empty() {
        let mut log_file = std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(log_file_path)
            .map_err(|e| Errx::e_io(e, format!("opening {DATABASE_NAME} for writing")))?;

        let subtitle = subtitle(time::year_month_day());

        write_to_database(&mut log_file, &format!("{TITLE}\n\n{subtitle}"))?;
    }

    Ok(log_file_path.to_path_buf())
}

fn create_log_file_path_name(xdg_data_home: &Path) -> Resultx<String> {
    let xdg_data_home = xdg_data_home
        .to_str()
        .ok_or_else(|| Errx::io("XDG_DATA_HOME is not valid str"))?;
    Ok(format!("{xdg_data_home}/{BIN_NAME}/{DATABASE_NAME}"))
}

fn format_content(
    mut new_content_lines: Vec<String>,
    mut old_content_lines: Vec<String>,
) -> String {
    let subtitle_current_day = subtitle(time::year_month_day());

    let new_content_contains_subtitle_current_day = new_content_lines
        .iter()
        .any(|line| line == &subtitle_current_day);
    let old_content_contains_subtitle_current_day = old_content_lines
        .iter()
        .any(|line| line == &subtitle_current_day);

    match (
        new_content_contains_subtitle_current_day,
        old_content_contains_subtitle_current_day,
    ) {
        (true, true) => {
            let idx_of_subtitle = old_content_lines
                .iter()
                .position(|line| line == &subtitle_current_day)
                .unwrap_or(0);

            // let new_lines_filtered = Vec::with_capacity(new_content_lines.len());

            let mut cnt = 1;
            for new_line in new_content_lines {
                old_content_lines.insert(idx_of_subtitle + cnt, new_line);
                cnt += 1;
            }

            old_content_lines.join("\n")
        }
        (true, false) => {
            // do nothing
            let new_content = new_content_lines.join("\n");
            let old_content = old_content_lines.join("\n");
            format!("{new_content}\n{old_content}")
        }
        (false, true) => {
            let idx_of_subtitle = old_content_lines
                .iter()
                .position(|line| line == &subtitle_current_day)
                .unwrap_or(0);

            let mut cnt = 1;
            for new_line in new_content_lines {
                old_content_lines.insert(idx_of_subtitle + cnt, new_line);
                cnt += 1;
            }
            old_content_lines.join("\n")
        }
        (false, false) => {
            new_content_lines = [subtitle_current_day.clone(), newline()]
                .into_iter()
                .chain(new_content_lines.into_iter())
                .collect();

            let idx_of_title = old_content_lines
                .iter()
                .position(|line| line == TITLE)
                .unwrap_or(0);

            let mut cnt = 1;
            for new_line in new_content_lines {
                old_content_lines.insert(idx_of_title + cnt, new_line);
                cnt += 1;
            }
            old_content_lines.join("\n")

            // let new_content = new_content_lines.join("\n");
            // let old_content = old_content_lines.join("\n");
            // format!("{new_content}\n{old_content}")
        }
    }
}

fn read_old_content(path: &Path) -> Resultx<String> {
    read_to_string(path).map_err(|e| Errx::e_io(e, "reading old database content"))
}

fn subtitle(day: String) -> String {
    format!("## {day}")
}

fn newline() -> String {
    "\n".to_string()
}

fn write_to_database(file: &mut File, content: &str) -> Resultx<()> {
    Write::write_all(file, content.as_bytes())
        .map_err(|e| Errx::e_io(e, format!("writing to {DATABASE_NAME}")))
}
