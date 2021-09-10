use std::path::Path;
use std::process::Command;

use crate::logging::*;
use console::{self, style};

pub fn handle(path: &str) {
    let mut lines = path.split("\n");
    let key = lines.next().unwrap().trim();
    let path = lines.next().unwrap();

    debug!("pressed key: {:#?}", key);

    match key {
        "ctrl-e" => edit(path),
        "" => run(path),
        _ => error!("unhandled result key: {:?}", key),
    }
}

fn run(path: &str) {
    let tip = format!("  Execute `{}`", path);
    println!("{}", style(tip).green());

    let child = Command::new(path).spawn();
    if let Err(ref error) = child {
        error!(
            "failed to execute action:\n  path: {:?}\n  error: {:?}",
            path, error
        );
    };

    child.unwrap().wait().unwrap();
}

pub fn edit<P: AsRef<Path>>(path: P) {
    let meta_path = path.as_ref().with_extension("toml");

    Command::new("nvim")
        .arg("-O")
        .arg(path.as_ref())
        .arg(meta_path)
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    // should `exec` it
    Command::new("ap").spawn().unwrap().wait().unwrap();
}
