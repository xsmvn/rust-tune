mod page;
mod model;
use iced::{Theme, border};
use iced::widget::button::background;
use iced::{Element, Task, Color};
use iced::widget::{button, column, container, row, Button};

use iced::Length;

use iced_aw::style;

use iced_aw::style::colors::{DARK, WHITE};
use page::{home::HomePage, profile::ProfilePage, settings::SettingsPage};

use crate::model::song::Song;
use crate::page::home;

// ==================== Pages ====================
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Page {
    Home,
    Profile,
    Settings,
}

#[derive(Debug, Clone)]
pub enum Message {
    GoToHome,
    GoToProfile,
    GoToSettings,
    // -----
    ThemeChanged(Theme),
}

struct RustTune {
    page_actuelle: Page,
    home_page: HomePage,
    profile_page: ProfilePage,
    settings_page: SettingsPage,
    theme_choosen: Theme,
}

fn transparent_button_style(t: Theme) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    move |_theme: &iced::Theme, _status: button::Status| {
        match t {
        Theme::Light => {
        button::Style {
            background: None,
            text_color:DARK,
            ..button::Style::default()}
        }
        Theme::Dark => {
            button::Style {
                background:None,
                text_color:WHITE,
                ..button::Style::default()
            }
        } 
        _ => {
            button::Style {
                ..button::Style::default()
            }
        }
    }
    }
}

fn new() -> (RustTune, Task<Message>) {
    let app = RustTune {
        page_actuelle: Page::Home,
        home_page: HomePage::new(),
        profile_page: ProfilePage::new(),
        settings_page: SettingsPage::new(),
        theme_choosen: Theme::Light,
    };
    (app, Task::none())
}

fn update(app: &mut RustTune, message: Message) -> Task<Message> {
    match message {
        Message::GoToHome
 => {
            app.page_actuelle = Page::Home;
        }
        Message::GoToProfile => {
            app.page_actuelle = Page::Profile;
        }
        Message::GoToSettings => {
            app.page_actuelle = Page::Settings;
        }
        Message::ThemeChanged(new_theme) => {
            app.theme_choosen = new_theme.clone();
            // better to make a method in settings_page to set the theme once and send message here
            app.settings_page.theme = new_theme;
        }
    }
    Task::none()
}


fn view<'a>(app: &'a RustTune) -> Element<'a,Message> {

    let page_content = match app.page_actuelle {
        Page::Home => app.home_page.view(),
        Page::Profile => app.profile_page.view(&app.home_page),
        Page::Settings => app.settings_page.view(),
    };
    let navigation = container(
        column![
            button("Accueil").style(transparent_button_style(app.theme_choosen.clone())).on_press(Message::GoToHome
    ),
            button("Profil").style(transparent_button_style(app.theme_choosen.clone())).on_press(Message::GoToProfile),
            button("Paramètres").style(transparent_button_style(app.theme_choosen.clone())).on_press(Message::GoToSettings),
        ]
        .spacing(12)
        .padding(20),
    )
    .height(Length::Fill);

    let content = container(page_content)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(10);

    let main_layout = row![navigation, content].spacing(15).height(Length::Fill);

    container(main_layout)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(10)
        .into()
}


fn theme(app: &RustTune) -> Theme {
    app.theme_choosen.clone()
}

// ====================== MAIN ======================
pub fn main() -> iced::Result {
    iced::application(new, update, view)
        .title("Rust Tune ♫")           
        .theme(theme)                   
        .window(iced::window::Settings {
            size: iced::Size::new(1000.0, 800.0),
            ..Default::default()
        })
        .run()
}