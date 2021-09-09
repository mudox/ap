use console::{self, pad_str, style, Alignment, Color};

use crate::model::{Action, ActionLocation::*};

pub struct Formatter<'a> {
    actions: &'a Vec<Action>,
}

impl<'a> Formatter<'a> {
    pub fn new(actions: &Vec<Action>) -> Formatter {
        Formatter { actions }
    }

    pub fn feed(&self) -> Vec<String> {
        self.actions
            .iter()
            .map(|a| self.line(a).to_string())
            .collect()
    }
}

impl<'a> Formatter<'a> {
    fn icon(&self, action: &Action) -> String {
        let icon = action.icon.clone().unwrap_or("·".to_string());
        let icon = style(icon).fg(icon_color(&action)).to_string();
        pad_str(&icon, 3, Alignment::Left, Some("")).to_string()
    }

    fn title(&self, action: &Action) -> String {
        let title = &action.title;
        pad_str(title, 100, Alignment::Left, None).to_string()
    }

    fn line(&self, action: &Action) -> String {
        let icon = self.icon(action);
        let title = self.title(action);

        let path = action.path.to_str().unwrap();
        format!(
            "{path}\t{icon} {title}",
            path = path,
            icon = icon,
            title = title
        )
    }
}

fn icon_color(action: &Action) -> Color {
    match &action.location() {
        Global => Color::Green,
        CurrentDirectory => Color::Yellow,
        AncestorDirectory => Color::Blue,
    }
}
