use std::path::Path;
use std::process::Command;

use console::{self, style};

use crate::logging::*;
use crate::model::Action;

pub fn handle(lines: &str, actions: &[Action]) {
    let mut lines = lines.split('\n');

    let key = lines.next().unwrap().trim();
    let line2 = lines.next().unwrap();
    let mut pair = line2.split('\t');
    let index = pair.next().unwrap().parse::<usize>();
    if let Err(e) = index {
        warn!("failed to parse index from line #2: {:#?}", e);
        return;
    }

    let action = &actions[index.unwrap()];

    if !action.path.exists() {
        println!("invalid action path: {:?}, quit", action.path);
        return;
    }

    debug!("pressed key: {:#?}", key);
    debug!("select path: {:#?}", action);

    match key {
        "ctrl-e" => edit_action(action),
        "" => run(action),
        _ => error!("unhandled result key: {:?}", key),
    }
}

fn run(action: &Action) {
    if let Some(ref tmux) = action.tmux {
        respawn_tmux_pane(action, &tmux.pane_title);
        return;
    }

    let tip = format!("  Execute `{:?}`", &action.path);
    println!("{}", style(tip).green());

    if action.cd.unwrap_or(false) {
        let parent_dir = action.path.parent().unwrap().parent().unwrap();
        std::env::set_current_dir(parent_dir).unwrap();
        let s = format!("at {:?}", parent_dir);
        let s = style(s).green();
        println!("{}", s);
    }

    let mut cmd = Command::new(&action.path);

    let child = cmd.spawn();
    if let Err(ref error) = child {
        error!(
            "failed to execute action:\n  path: {:?}\n  error: {:?}",
            action.path, error
        );
    };

    child.unwrap().wait().unwrap();
}

fn respawn_tmux_pane(action: &Action, title: &str) {
    // check if is in tmux
    if std::env::var("TMUX_PANE").is_err() {
        return;
    }

    let pane_id = std::env::var("TMUX_PANE").unwrap();

    // set pane title
    tmux_run(&[
        "set-option",
        "-p",
        "-t",
        &pane_id,
        "pane-border-format",
        &format!(" [#{{pane_index}}] {}", title),
    ]);

    // respawn pane
    // NOTE: `respawn-pane` below would kill current process, hence code after it is not reachable
    let path = action.path.to_str().unwrap();
    tmux_run(&["respawn-pane", "-k", path]);
}

fn tmux_run(args: &[&str]) {
    let child = Command::new("tmux").args(args).spawn();

    if let Err(ref error) = child {
        error!(
            "failed to run tmux command:\n  args: {:?}\n  error: {:?}",
            args, error
        );
    };

    child.unwrap().wait().unwrap();
}

pub fn edit_action(action: &Action) {
    edit(&action.path);
}

pub fn edit<P: AsRef<Path>>(path: &P) {
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
