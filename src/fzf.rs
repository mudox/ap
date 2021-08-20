use termion::color::{self, Color};
use termkit::ui::*;

use crate::model::{Action, ActionLocation};

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
        lspan(&icon, icon_color(&action), 3)
    }

    fn title(&self, action: &Action) -> String {
        let title = &action.title;
        lspan(title, color::White, 3)
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

fn icon_color(action: &Action) -> &'static dyn Color {
    match &action.location() {
        ActionLocation::Global => &color::Green,
        ActionLocation::CurrentDirectory => &color::Yellow,
        ActionLocation::AncestorDirectory => &color::Blue,
    }
}
