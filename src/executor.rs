use std::process::Command;

use termion::color;
use termkit::ui::fg;

use crate::logging::*;

pub fn execute(path: &str) {
    let tip = format!("executing `{}` ...", path);
    let tip = fg(color::Green, &tip);
    println!("{}", tip);

    let child = Command::new(path).spawn();
    if let Err(ref error) = child {
        error!(
            "failed to execute action:\n  path: {:?}\n  error: {:?}",
            path, error
        );
    };

    child.unwrap().wait().unwrap();
}
