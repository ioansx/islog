use std::path::Path;

pub fn open_neovim(file_path: &Path) {
    let child = std::process::Command::new("nvim")
        .arg(file_path)
        .spawn()
        .expect("Failed to open Neovim");

    if let Err(e) = child.wait_with_output() {
        eprintln!("Neovim process failed: {}", e);
    }
}
