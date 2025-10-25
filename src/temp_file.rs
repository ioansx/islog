use std::path::{Path, PathBuf};

use crate::constants::BIN_NAME;
use crate::error::{Errx, Resultx};
use crate::{random::random_seed, time::now_timestamp_nanos};

pub struct TempFile {
    path: PathBuf,
}

impl TempFile {
    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn contents(&self) -> Resultx<String> {
        std::fs::read_to_string(&self.path)
            .map_err(|e| Errx::e_io(e, format!("reading {:?}", self.path)))
    }
}

impl Default for TempFile {
    fn default() -> Self {
        let now = now_timestamp_nanos();
        let ramdon_seed = random_seed();
        let path_str = format!("/tmp/{BIN_NAME}-{now}-{ramdon_seed}.txt");
        let path = PathBuf::from(&path_str);
        TempFile { path }
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        if !self.path.exists() {
            return;
        }

        if let Err(e) = std::fs::remove_file(&self.path) {
            eprintln!("should remove temporaty file: {e}");
        }
    }
}
