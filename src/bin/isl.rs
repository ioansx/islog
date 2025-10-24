use std::path::PathBuf;

use islog::{
    constants::{HOME, XDG_DATA_HOME},
    database::Database,
    nvim::open_neovim,
    temp_file::create_temp_file,
    xdg::default_xdg_data_home,
};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let xdg_data_home = std::env::var(XDG_DATA_HOME)
        .map(|x| PathBuf::from(&x))
        .unwrap_or_else(|_| {
            let home = std::env::var(HOME).expect("HOME environment variable not set");
            default_xdg_data_home(&home)
        });

    let database = Database::init(&xdg_data_home);

    multiplex(args, database);
}

fn multiplex(args: Vec<String>, database: Database) {
    match (args.len(), args.get(1).map(|x| x.as_str())) {
        (1, _) => {
            let temp_file_path = create_temp_file();

            open_neovim(&temp_file_path);

            let temp_file = std::fs::read_to_string(&temp_file_path)
                .expect("failed to read temporary file content");

            std::fs::remove_file(&temp_file_path).expect("failed to remove temporary file");

            if temp_file.trim().is_empty() {
                return;
            }

            database.update(temp_file);
        }
        (2, Some("version")) | (2, Some("--version")) => {
            let version = env!("CARGO_PKG_VERSION");
            println!("islog version {version}");
            return;
        }
        (2, Some("help")) | (2, Some("--help")) => {
            println!("islog - A simple command-line logging tool using neovim");
            println!();
            println!("Usage:");
            println!("  islog           Open Neovim to add a new log entry.");
            println!("  islog --help    Show this help message.");
            println!("  islog --show    Show the log database.");
            println!("  islog --version Show version information.");
            return;
        }
        (2, Some("show")) | (2, Some("--show")) => {
            open_neovim(database.path());
        }
        _ => {
            println!("Unknown command. Use --help for usage information.");
        }
    }
}
