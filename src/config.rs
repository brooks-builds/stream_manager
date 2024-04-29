use std::path::{Path, PathBuf};

pub struct Config {
    pub helix_config_path: PathBuf,
}

impl Config {
    pub fn new(helix_config_path: impl AsRef<Path>) -> Self {
        let helix_config_path = helix_config_path.as_ref().to_path_buf();

        Self { helix_config_path }
    }
}
