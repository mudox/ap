use std::path::{Path, PathBuf};

use clap::{app_from_crate, App, AppSettings, Arg};

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
    New { name: String, global: bool },
    Execute,
    Preview(String),
}

pub struct Config {
    pub task: Task,
}

impl Config {
    pub fn load() -> Config {
        let new = App::new("new")
            .visible_aliases(&["a", "n"])
            .about("Create new action")
            .arg(
                Arg::new("global")
                    .short('g')
                    .long("global")
                    .about("Create a global action"),
            )
            .arg(
                Arg::new("ACTION_NAME")
                    .about("The filename of the action")
                    .required(true)
                    .index(1),
            );

        let preview = App::new("preview")
            .visible_alias("p")
            .about("Generate fzf preview content for ACTION_PATH")
            .arg(
                Arg::new("ACTION_PATH")
                    .about("The path of the action file to generate preview")
                    .required(true)
                    .index(1),
            )
            .setting(AppSettings::Hidden);

        let matches = app_from_crate!()
            .subcommand(new)
            .subcommand(preview)
            .get_matches();

        let task = if let Some(matches) = matches.subcommand_matches("preview") {
            let path = matches.value_of("ACTION_PATH").unwrap().to_string();
            Task::Preview(path)
        } else if let Some(matches) = matches.subcommand_matches("new") {
            let name = matches.value_of("ACTION_NAME").unwrap().to_string();
            let global = matches.is_present("global");
            Task::New { name, global }
        } else {
            Task::Execute
        };

        Config { task }
    }
}
