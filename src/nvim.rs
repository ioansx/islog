pub fn open_neovim(temp_file: &str) {
    let child = std::process::Command::new("nvim")
        .arg(temp_file)
        .spawn()
        .expect("Failed to open Neovim");
    let _r = child.wait_with_output();
}
