use std::path::PathBuf;

use islog::{
    constants::{HOME, PKG_NAME, XDG_DATA_HOME},
    database::Database,
    error::Resultx,
    nvim::open_neovim,
    temp_file::TempFile,
    xdg::default_xdg_data_home,
};

fn main() -> Resultx<()> {
    let args = std::env::args().collect::<Vec<String>>();

    let xdg_data_home = std::env::var(XDG_DATA_HOME)
        .map(|x| PathBuf::from(&x))
        .unwrap_or_else(|_| {
            let home = std::env::var(HOME).expect("HOME environment variable not set");
            default_xdg_data_home(&home)
        });

    let database = Database::init(&xdg_data_home)?;

    if let Err(e) = multiplex(args, database) {
        e.log();
    }

    Ok(())
}

fn multiplex(args: Vec<String>, database: Database) -> Resultx<()> {
    match (args.len(), args.get(1).map(|x| x.as_str())) {
        (1, _) => {
            let temp_file = TempFile::default();
            open_neovim(temp_file.path())?;

            let temp_file_contents = temp_file.contents()?;
            if temp_file_contents.trim().is_empty() {
                return Ok(());
            }

            database.update(temp_file_contents)?;
        }
        (2, Some("show")) | (2, Some("--show")) => {
            open_neovim(database.path())?;
        }
        (2, Some("version")) | (2, Some("--version")) => {
            let version = env!("CARGO_PKG_VERSION");
            println!("{PKG_NAME} version {version}");
        }
        (2, Some("help")) | (2, Some("--help")) => {
            println!("{PKG_NAME} - A simple command-line logging tool using neovim");
            println!();
            println!("Usage:");
            println!("  {PKG_NAME}           Open Neovim to add a new log entry.");
            println!("  {PKG_NAME} --help    Show this help message.");
            println!("  {PKG_NAME} --show    Show the log database.");
            println!("  {PKG_NAME} --version Show version information.");
        }
        _ => {
            println!("Unknown command. Use --help for usage information.");
        }
    }

    Ok(())
}
