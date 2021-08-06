use std::error::Error;
use std::fs;
use std::process::Command;
use std::str;

use termion::color;
use termkit::ui::*;
use time::{format_description, OffsetDateTime};

use crate::logging::*;
use crate::model::Action;

/// Generate and print preview for action to stdout.
pub fn preview(path: &str) {
    let action = Action::load_from(path);
    if action.is_none() {
        return;
    }
    let action = action.unwrap();

    // path
    println!("{}", line("Path", path));

    // file type
    if let Some(content) = filetype(path) {
        println!("{}", line("File Type", &content));
    }

    // creation time
    if let Ok(ctime) = ctime(path) {
        println!("{}", line("Created", &ctime));
    }

    // description
    if let Some(desc) = action.description {
        println!("{}", line("Description", &desc));
    }

    // bat filter
    if let Ok(bat) = bat(path) {
        println!("{}", &bat);
    }
}

/// The width of first field
const W0: usize = 16;

fn line(title: &str, content: &str) -> String {
    let title = span(title, color::Blue, W0, Alignment::Left);
    let content = fg(color::Yellow, content);
    format!("{}{}", title, content)
}

fn filetype(path: &str) -> Option<String> {
    let output = Command::new("file").arg("--brief").arg(path).output();
    if let Err(error) = output {
        warn!(
            "failed to invoke `file` command:\n  path: {:?}\n  error: {:#?}",
            path, error,
        );
        return None;
    }

    String::from_utf8(output.unwrap().stdout)
        .ok()
        .map(|x| x.trim().to_string())
}

fn ctime(path: &str) -> Result<String, Box<dyn Error>> {
    let meta = fs::metadata(path)?;
    let ctime: OffsetDateTime = meta.created()?.into();
    let format = format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]")?;
    Ok(ctime.format(&format)?)
}

fn bat(path: &str) -> Result<String, Box<dyn Error>> {
    let output = Command::new("bat")
        .arg("--color=always")
        .arg("--style=grid,numbers,rule,snip,changes")
        .arg("--terminal-width=114")
        .arg("--wrap=never")
        .arg(path)
        .output();
    Ok(String::from_utf8(output?.stdout)?)
}
