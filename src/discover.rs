use std::ffi::OsStr;
use std::fs;
use std::path::Path;

use crate::logging::*;

use crate::config;
use crate::model::Action;

/// Lookup actions from `.ap-actions` directory under the argument `path`.
fn actions_from<P: AsRef<Path>>(path: P) -> Vec<Action> {
    let entries = fs::read_dir(&path);
    if let Err(error) = entries {
        info!(
            "failed at `read_dir`:\n  path: {:?}\n  error: {:#?}",
            path.as_ref(),
            error,
        );
        return Vec::new();
    }

    entries
        .unwrap() // checked above
        .filter_map(|entry| {
            if entry.is_err() {
                error!("failed to unwrap entry: {:?}", &entry);
                return None;
            }

            let entry = entry.unwrap();
            if entry.path().extension() == Some(OsStr::new("toml")) {
                info!("skip toml file: {:?}", &entry.path());
                return None;
            }

            Action::load_from(entry.path())
        })
        .collect()
}

fn global_actions() -> Vec<Action> {
    let path = config::global_actions_dir();
    actions_from(&path)
}

fn local_actions<P: AsRef<Path>>(path: &P) -> Vec<Action> {
    path.as_ref()
        .ancestors()
        .map(|path| actions_from(path.join(".ap-actions")))
        .flatten()
        .collect()
}

pub fn actions() -> Vec<Action> {
    let mut actions = Vec::new();

    if let Ok(path) = std::env::current_dir() {
        actions.extend(local_actions(&path));
    }

    actions.extend(global_actions());

    actions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn test_local_actions() {
        let path = Path::new("/Users/mudox/Develop/Rust/ap/tests/dir/d0/d1/d2/d3");
        let actions = local_actions(&path);
        println!("actions: {:#?}", &actions);

        let ac1 = Action {
            path: path
                .parent()
                .unwrap()
                .parent()
                .unwrap()
                .join(".ap-actions/ac1"),
            icon: Some("ac1".to_string()),
            title: "Title of ac1".to_string(),
            description: Some("Description of ac1\n".to_string()),
            tmux: None,
            cd: None,
        };

        let ac3 = Action {
            path: path.join(".ap-actions/ac3"),
            icon: Some("ac3".to_string()),
            title: "Title of ac3".to_string(),
            description: Some("Description of ac3\n".to_string()),
            tmux: None,
            cd: None,
        };

        assert_eq!(actions[0], ac3);
        assert_eq!(actions[1], ac1);
    }
}
