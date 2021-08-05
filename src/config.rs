use std::path::{Path, PathBuf};

use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg, SubCommand};

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

pub enum Task {
    List,
    Preview(String),
}

pub struct Config {
    pub task: Task,
}

impl Config {
    pub fn load() -> Config {
        let matches = App::new(crate_name!())
            .author(crate_authors!())
            .version(crate_version!())
            .about(crate_description!())
            .subcommand(
                SubCommand::with_name("preview")
                    .alias("p")
                    .about("Generate fzf preview content for ACTION_PATH")
                    .arg(
                        Arg::with_name("ACTION_PATH")
                            .help("The path of the action file to generate preview")
                            .required(true)
                            .index(1),
                    ),
            )
            .get_matches();

        let task = if let Some(matches) = matches.subcommand_matches("preview") {
            let path = matches.value_of("ACTION_PATH").unwrap().to_string();
            Task::Preview(path)
        } else {
            Task::List
        };

        Config { task }
    }
}
