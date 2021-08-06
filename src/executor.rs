use std::process::Command;

use termion::color;
use termkit::ui::fg;

use crate::logging::*;

pub fn execute(path: &str) {
    let tip = format!("executing `{}` ...", path);
    let tip = fg(color::Green, &tip);
    println!("{}", tip);

    if let Err(error) = Command::new(path).spawn() {
        error!(
            "failed to execute action:\n  path: {:?}\n  error: {:?}",
            path, error
        );
    };
}
