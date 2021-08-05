use std::ffi::OsStr;
use std::fs;
use std::path::Path;

use crate::logging::*;

use crate::config::{self};
use crate::model::Action;

/// Lookup actions from `.ap-actions` directory under the argument `path`.
pub fn actions<P: AsRef<Path>>(path: P) -> Vec<Action> {
    let entries = fs::read_dir(&path);
    if let Err(error) = entries {
        warn!(
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

pub fn global_actions() -> Vec<Action> {
    let path = config::global_actions_dir();
    actions(&path)
}

pub fn local_actions<P: AsRef<Path>>(path: &P) -> Vec<Action> {
    path.as_ref()
        .ancestors()
        .map(|path| actions(path.join(".ap-actions")))
        .flatten()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};

    use std::path::PathBuf;

    #[test]
    fn test_local_actions() {
        let path = Path::new("/Users/mudox/Develop/Rust/ap/tests/dir/d0/d1/d2/d3");
        let actions = local_actions(&path);
        println!("actions: {:#?}", &actions);

        let ac1 = Action {
            path: PathBuf::from(
                path.parent()
                    .unwrap()
                    .parent()
                    .unwrap()
                    .join(".ap-actions/ac1"),
            ),
            icon: Some("ac1".to_string()),
            title: "Title of ac1".to_string(),
            description: Some("Description of ac1\n".to_string()),
        };

        let ac3 = Action {
            path: PathBuf::from(path.join(".ap-actions/ac3")),
            icon: Some("ac3".to_string()),
            title: "Title of ac3".to_string(),
            description: Some("Description of ac3\n".to_string()),
        };

        assert_eq!(actions[0], ac3);
        assert_eq!(actions[1], ac1);
    }
}
