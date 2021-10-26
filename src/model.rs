use std::fs;
use std::path::Path;
use std::path::PathBuf;

use is_executable::is_executable;
use serde::Deserialize;

use crate::config;
use crate::logging::*;

pub enum ActionLocation {
    Global,
    CurrentDirectory,
    AncestorDirectory,
}

#[derive(Default, Debug, Deserialize, PartialEq)]
pub struct Action {
    #[serde(skip)]
    pub path: PathBuf,

    #[serde(default)]
    pub icon: Option<String>,

    pub title: String,

    pub description: Option<String>,

    pub cd: Option<bool>,
}

impl Action {
    /// Construct a `Action` from input `path` and its corresponding toml file.
    ///
    /// Argument `path` must be executable and has a toml file named `{stem}.toml` under the
    /// same directory
    pub fn load_from<P: AsRef<Path>>(path: P) -> Option<Action> {
        let path = path.as_ref();
        trace!("Action::load_from: {:?}", &path);

        if !is_executable(&path) {
            debug!("skip non-executable file: {:?}", &path);
            return None;
        }

        let meta_path = path.with_extension("toml");
        let text = fs::read_to_string(&meta_path);
        if let Err(e) = text {
            error!(
                "failed to read file\n  path: {:#?}\n  error: {:#?}",
                &meta_path, &e
            );
            return None;
        }
        let text = text.unwrap();

        let action = toml::from_str::<Action>(&text);
        if let Err(error) = action {
            error!(
                "failed to parse toml file\n  path: {:#?}\n  error: {:#?}",
                &meta_path, &error
            );
            return None;
        }

        let mut action = action.unwrap();
        action.path = path.to_path_buf();

        info!("found action: {:?}", action.path);
        Some(action)
    }

    pub fn location(&self) -> ActionLocation {
        let path = self.path.parent().unwrap();

        if path == config::global_actions_dir().as_path() {
            ActionLocation::Global
        } else if path.parent().unwrap() == std::env::current_dir().unwrap().as_path() {
            ActionLocation::CurrentDirectory
        } else {
            ActionLocation::AncestorDirectory
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn test_action_load() {
        let path = "/Users/mudox/Develop/Rust/ap/tests/dir/.ap-actions/can-run";
        let left = Action::load_from(path).unwrap();

        let right = Action {
            path: PathBuf::from(path),
            icon: Some("\u{f592} ".to_string()),
            title: "Title of can-run".to_string(),
            description: Some("Description of can-run\n".to_string()),
            ..Default::default()
        };

        assert_eq!(left, right);
    }
}
