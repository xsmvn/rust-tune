use iced::{Element, widget::{column, pick_list, text}, Theme};
use crate::Message;

#[derive(Clone, Debug)]
pub struct SettingsPage {
    pub theme: Theme,
}

impl SettingsPage {
    pub fn new() -> Self {
        SettingsPage {
            theme: Theme::Light,
        }
    }

    pub fn view<'a>(&'a self) -> Element<'a, crate::Message> {
        // themes taken from https://docs.iced.rs/iced/enum.Theme.html
        let themes = vec![
            Theme::Light,
            Theme::Dark,
            Theme::CatppuccinMacchiato,
        ];

        column![
            text("Page Paramètres").size(24),
            text("Choisissez votre thème:").size(18),
            pick_list(themes, Some(self.theme.clone()), |theme| Message::ThemeChanged(theme)),
        ]
        .spacing(20)
        .into()
    }
}