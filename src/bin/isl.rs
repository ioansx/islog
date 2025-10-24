use std::{fs::File, path::Path};

use islog::{
    constants::{HOME, XDG_DATA_HOME},
    create_temp_file_name,
    log_file::create_log_file_path_name,
    nvim::open_neovim,
    xdg::default_xdg_data_home,
};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() > 1 && args[1] == "--version" {
        let version = env!("CARGO_PKG_VERSION");
        println!("islog version {version}");
        return;
    }

    if args.len() > 1 && args[1] == "--help" {
        println!("islog - A simple command-line logging tool using neovim");
        println!();
        println!("Usage:");
        println!("  islog           Open Neovim to add a new log entry.");
        println!("  islog --help    Show this help message.");
        println!("  islog --show    Show the log database.");
        println!("  islog --version Show version information.");
        return;
    }

    let xdg_data_home = std::env::var(XDG_DATA_HOME).unwrap_or_else(|_| {
        let home = std::env::var(HOME).expect("HOME environment variable not set");
        default_xdg_data_home(&home)
    });

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

    if args.len() > 1 && args[1] == "--show" {
        open_neovim(&log_file_path_name);
        return;
    }

    let temp_file_name = create_temp_file_name();
    open_neovim(&temp_file_name);

    let temp_file =
        std::fs::read_to_string(&temp_file_name).expect("failed to read temporary file content");

    if temp_file.trim().is_empty() {
        std::fs::remove_file(&temp_file_name).expect("failed to remove temporary file");
        return;
    }

    let log_file_content =
        std::fs::read_to_string(&log_file_path).expect("failed to read log file content");

    let mut log_file = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&log_file_path)
        .expect("failed to open log file for appending");

    std::io::Write::write_all(&mut log_file, temp_file.as_bytes())
        .expect("failed to write to log file");
    std::io::Write::write_all(&mut log_file, b"\n").expect("failed to write newline to log file");
    std::io::Write::write_all(&mut log_file, log_file_content.as_bytes())
        .expect("failed to append old log file content");

    std::fs::remove_file(&temp_file_name).expect("failed to remove temporary file");
}
