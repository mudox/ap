use term_kit as term;
use termion::color;

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
        term::span(&icon, color::Yellow, 3, term::Alignment::Left)
    }

    fn title(&self, action: &Action) -> String {
        let title = &action.title;
        term::span(title, color::White, 3, term::Alignment::Left)
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

impl<'a> Formatter<'a> {
    pub fn preview(&self, action: &Action) -> String {
        unimplemented!()
    }
}
