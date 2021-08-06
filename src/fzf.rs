use termion::color;
use termkit::ui::*;

use crate::model::Action;

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
        lspan(&icon, color::Yellow, 3)
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
