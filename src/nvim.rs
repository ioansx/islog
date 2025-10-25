use std::path::Path;

use crate::error::{Errx, Resultx};

pub fn open_neovim(file_path: &Path) -> Resultx<()> {
    let child_result = std::process::Command::new("nvim")
        .arg(file_path)
        .spawn()
        .map_err(|e| Errx::e_io(e, "neovim should open; is it installed?"));

    match child_result {
        Ok(mut child) => {
            let status = child
                .wait()
                .map_err(|e| Errx::e_io(e, "waiting neovim process"))?;
            if status.success() {
                Ok(())
            } else {
                Err(Errx::g(format!("neovim exited with an error: {status}")))
            }
        }
        Err(e) => Err(e),
    }
}
