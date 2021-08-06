use std::io::Write;
use std::process::{Command, Stdio};
use std::str;

use crate::config::{Config, Task};
use crate::discover;
use crate::executor::execute;
use crate::fzf::Formatter;
use crate::logging::*;
use crate::model::Action;
use crate::preview::preview;

pub fn run(config: Config) {
    match config.task {
        Task::List => {
            let actions = discover::actions();
            match choose_action(&actions) {
                Some(path) => execute(&path),
                _ => info!("nothing selected"),
            }
        }
        Task::Preview(path) => preview(&path),
    }
}

fn choose_action(actions: &Vec<Action>) -> Option<String> {
    let fzf = Formatter::new(actions);
    let feed = fzf.feed().join("\n");

    let mut cmd = Command::new("fzf");
    let cmd_ref = cmd
        .arg("--with-nth=2..")
        .arg("--no-sort")
        .arg("--tiebreak=end")
        .arg("--ansi")
        .arg("--margin=2")
        .arg("--inline-info")
        .arg("--header")
        .arg("") // sepratate line
        .arg("--prompt=▶ ")
        .arg("--pointer=▶")
        .arg("--color=bg:-1,bg+:-1") // transparent background
        // .arg("--border=none")
        .arg("--preview")
        .arg("ap preview {1}")
        .arg("--preview-window")
        // .arg("right,60%,wrap,border-none");
        .arg("right,60%,wrap");

    let mut child = cmd_ref
        // pipe
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        // run
        .spawn()
        .expect("failed to spawn `fzf` command");

    child
        .stdin
        .take()
        .unwrap()
        .write_all(&feed.into_bytes())
        .expect("failed to write to stdin of `fzf` command");

    let output = child
        .wait_with_output()
        .expect("failed to wait `fzf` to exit");
    let output = str::from_utf8(output.stdout.as_slice()).unwrap();

    let path = output.split("\t").take(1).collect::<String>();
    debug!("chosen path: {:?}", path);

    if path.is_empty() {
        None
    } else {
        Some(path)
    }
}
