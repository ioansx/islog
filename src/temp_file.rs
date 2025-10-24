use std::path::{Path, PathBuf};

use crate::constants::BIN_NAME;
use crate::{random::random_seed, time::now_timestamp_nanos};

pub fn create_temp_file() -> PathBuf {
    let now = now_timestamp_nanos();
    let ramdon_seed = random_seed();
    let path = format!("/tmp/{BIN_NAME}-{now}-{ramdon_seed}.txt");
    PathBuf::from(Path::new(&path))
}
