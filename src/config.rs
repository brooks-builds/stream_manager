use std::path::{Path, PathBuf};

#[derive(Clone)]
pub struct Config {
    pub helix_config_path: PathBuf,
    pub alacritty_font_config_path: PathBuf,
}

impl Config {
    pub fn new(
        helix_config_path: impl AsRef<Path>,
        alacritty_font_config_path: impl Into<PathBuf>,
    ) -> Self {
        let helix_config_path = helix_config_path.as_ref().to_path_buf();

        Self {
            helix_config_path,
            alacritty_font_config_path: alacritty_font_config_path.into(),
        }
    }
}
