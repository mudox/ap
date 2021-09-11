use console::Term;

use std::fs::{self, OpenOptions};
use std::io::{ErrorKind, Write};
use std::os::unix::fs::OpenOptionsExt;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::str;

use crate::config::{global_actions_dir, Config, Task};
use crate::discover;
use crate::executor;
use crate::fzf::Formatter;
use crate::logging::*;
use crate::model::Action;
use crate::preview::preview;

pub fn run(config: Config) {
    match config.task {
        Task::Execute => {
            let actions = discover::actions();
            match choose_action(&actions) {
                Some(lines) => executor::handle(&lines),
                _ => info!("nothing selected"),
            }
        }
        Task::New { name, global } => create_action(&name, global),
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
        .arg("Ctrl-e: edit") // sepratate line
        .arg("--prompt=▶ ")
        .arg("--pointer=▶")
        .arg("--color=bg:-1,bg+:-1"); // transparent background

    // preview
    cmd_mut_ref = cmd_mut_ref
        .arg("--preview")
        .arg("ap preview {1}")
        .arg("--preview-window");

    if let Some((_, w)) = Term::stdout().size_checked() {
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

    cmd_mut_ref = cmd_mut_ref.arg("--expect=ctrl-e");

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

const SCRIPT_TEMPLATE: &str = "\
#!/usr/bin/env zsh

echo 'Hello world!'
";

const META_TEMPLATE: &str = "\
# icon = \"+\"

# required
title = \"Hello world\"

# description = \"\"
";

fn create_action(name: &str, global: bool) {
    // determine path
    let dir = if global {
        global_actions_dir()
    } else {
        choose_local_action_dir()
    };

    // create `.ap-actions` dir if not exists
    if let Err(error) = fs::create_dir(&dir) {
        if error.kind() != ErrorKind::AlreadyExists {
            error!(
                "failed to create directory:\npath  {:?}\n  error: {:#?}",
                dir, error
            );
            return;
        }
    }

    // populate script file if not exists
    let path = dir.join(name);
    if !path.exists() {
        OpenOptions::new()
            .create(true)
            .write(true)
            .mode(0o731)
            .open(&path)
            .unwrap()
            .write_all(SCRIPT_TEMPLATE.as_bytes())
            .unwrap();
    }

    // populate meta file if not exists
    let meta_path = path.with_extension("toml");
    if !meta_path.exists() {
        OpenOptions::new()
            .create(true)
            .write(true)
            .open(&meta_path)
            .unwrap()
            .write_all(META_TEMPLATE.as_bytes())
            .unwrap();
    }

    // open in editor
    crate::executor::edit(&path);
}

fn choose_local_action_dir() -> PathBuf {
    std::env::current_dir().unwrap().join(".ap-actions")
}
