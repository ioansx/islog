use std::path::Path;

pub fn open_neovim(file_path: &Path) {
    let child = std::process::Command::new("nvim")
        .arg(file_path)
        .spawn()
        .expect("neovim should open; is it installed?");

    if let Err(e) = child.wait_with_output() {
        eprintln!("neovim process failed: {e}");
    }
}
