use std::io::Write;
use std::process::{Command, Stdio};
use std::str;

use crate::config::{Config, Task};
use crate::discover;
use crate::executor;
use crate::fzf::Formatter;
use crate::logging::*;
use crate::model::Action;
use crate::preview::preview;

pub fn run(config: Config) {
    match config.task {
        Task::List => {
            let actions = discover::actions();
            match choose_action(&actions) {
                Some(lines) => executor::handle(&lines),
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
    let mut cmd_mut_ref = cmd.env("FZF_DEFAULT_OPTS", "");

    // search
    cmd_mut_ref = cmd_mut_ref
        .arg("--with-nth=2..")
        .arg("--no-sort")
        .arg("--tiebreak=end");

    // appearance
    cmd_mut_ref = cmd_mut_ref
        .arg("--layout=reverse")
        .arg("--height=60%")
        .arg("--min-height=30")
        .arg("--ansi")
        .arg("--margin=1")
        .arg("--padding=1")
        .arg("--inline-info")
        .arg("--header")
        .arg("Ctrl-e: edit, Ctrl-i: edit info") // sepratate line
        .arg("--prompt=▶ ")
        .arg("--pointer=▶")
        .arg("--color=bg:-1,bg+:-1"); // transparent background

    // preview
    cmd_mut_ref = cmd_mut_ref
        .arg("--preview")
        .arg("ap preview {1}")
        .arg("--preview-window");

    if let Ok((w, _)) = termion::terminal_size() {
        // 💀 magic number */
        if w < 170 {
            cmd_mut_ref.arg("down,70%,nowrap");
        } else {
            cmd_mut_ref.arg("right,60%,nowrap");
        }
    }

    // key bindings
    cmd_mut_ref = cmd_mut_ref
        .arg("--bind")
        .arg("ctrl-f:page-down")
        .arg("--bind")
        .arg("ctrl-b:page-up")
        .arg("--bind")
        .arg("ctrl-alt-f:preview-page-down")
        .arg("--bind")
        .arg("ctrl-alt-b:preview-page-up");

    cmd_mut_ref = cmd_mut_ref.arg("--expect=ctrl-e,ctrl-i");

    let mut child = cmd_mut_ref
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

    // would get 2 lines if fzf not cancelled by user:
    //   1 - the key pressed to end fzf, empty for `enter`
    //   2 - the path of the chosen action
    let lines = output.split("\t").take(1).collect::<String>();
    debug!("chosen: {:?}", lines);

    if lines.is_empty() {
        None
    } else {
        Some(lines)
    }
}
