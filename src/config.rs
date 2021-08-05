use std::path::{Path, PathBuf};

pub struct Config {}

/// Return config dir.
pub fn dir() -> PathBuf {
    let mut path = dirs::home_dir().unwrap();
    path.push(".config/ap");

    match std::env::var("XDG_CONFIG_HOME") {
        Ok(path) => Path::new(&path).join("ap").to_path_buf(),
        _ => path,
    }
}

/// Global actions directory: `<confi_dir>/actions`
pub fn global_actions_dir() -> PathBuf {
    dir().join("actions")
}

impl Config {
    pub fn load() -> Config {
        Config {}
    }
}
