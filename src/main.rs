mod page;
mod model;
use iced::{Theme, border};
use iced::widget::button::background;
use iced::{Element, Task, Color};
use iced::widget::{button, column, container, row, Button, text};

use iced::Length;

use iced_aw::style;
use iced_aw::style::colors::{DARK, WHITE};
use page::{home::HomePage, profile::ProfilePage, settings::SettingsPage};

use crate::model::song::Song;
use crate::page::home;

// ==================== RODIO 0.22 (avec playback) ====================
use rodio::{Decoder, Player, MixerDeviceSink};
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};

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
    ThemeChanged(Theme),
    Play,
    Pause,
    NextSong,
    PreviousSong,
    PlaySong(String), // Lance un son depuis home
}

struct RustTune {
    page_actuelle: Page,
    home_page: HomePage,
    profile_page: ProfilePage,
    settings_page: SettingsPage,
    theme_choosen: Theme,
    player: Arc<Mutex<Option<Player>>>,
    _stream: Arc<Mutex<Option<MixerDeviceSink>>>,
    current_song: Option<String>,
    is_playing: bool,
}

fn transparent_button_style(t: Theme) -> impl Fn(&iced::Theme, button::Status) -> button::Style {
    move |_theme: &iced::Theme, _status: button::Status| {
        match t {
            Theme::Light => button::Style {
                background: None,
                text_color: DARK,
                ..button::Style::default()
            },
            Theme::Dark => button::Style {
                background: None,
                text_color: WHITE,
                ..button::Style::default()
            },
            _ => button::Style::default(),
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
        player: Arc::new(Mutex::new(None)),
        _stream: Arc::new(Mutex::new(None)),
        current_song: None,
        is_playing: false,
    };
    (app, Task::none())
}

fn update(app: &mut RustTune, message: Message) -> Task<Message> {
    match message {
        Message::GoToHome => app.page_actuelle = Page::Home,
        Message::GoToProfile => app.page_actuelle = Page::Profile,
        Message::GoToSettings => app.page_actuelle = Page::Settings,
        Message::ThemeChanged(new_theme) => {
            app.theme_choosen = new_theme.clone();
            app.settings_page.theme = new_theme;
        }
        Message::Play => {
            if let Some(p) = app.player.lock().unwrap().as_ref() {
                p.play();
                app.is_playing = true;
            }
        }
        Message::Pause => {
            if let Some(p) = app.player.lock().unwrap().as_ref() {
                p.pause();
                app.is_playing = false;
            }
        }
        // a faire !!!!!
        Message::NextSong => {
            println!("▶ Next song"); 
        }
        Message::PreviousSong => {
            println!("◀ Previous song");
        }
        Message::PlaySong(path) => {
            app.play_song(&path);
        }
    }
    Task::none()
}

// ==================== BARRE D'AUDIO ====================
fn player_bar<'a>(app: &'a RustTune) -> Element<'a, Message> {
    let title = app.current_song.as_deref().unwrap_or("Aucune piste");


    row![
        button(text("⏮")).style(transparent_button_style(app.theme_choosen.clone())).on_press(Message::PreviousSong),
        if app.is_playing {
            button("⏸").style(transparent_button_style(app.theme_choosen.clone())).on_press(Message::Pause)
        } else {
            button(text("▶")).style(transparent_button_style(app.theme_choosen.clone())).on_press(Message::Play)
        },
        button(text("⏭")).style(transparent_button_style(app.theme_choosen.clone())).on_press(Message::NextSong),
        text(title).size(16).width(Length::Fill),
    ]
    .spacing(16)
    .padding(16)
    .align_y(iced::Alignment::Center)
    .into()
}

fn view<'a>(app: &'a RustTune) -> Element<'a, Message> {
    let page_content = match app.page_actuelle {
        Page::Home => app.home_page.view(),
        Page::Profile => app.profile_page.view(&app.home_page),
        Page::Settings => app.settings_page.view(),
    };

    let navigation = container(
        column![
            button("Accueil").style(transparent_button_style(app.theme_choosen.clone())).on_press(Message::GoToHome),
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

    container(
        column![
            main_layout,
            player_bar(app),
        ]
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .padding(10)
    .into()
}

fn theme(app: &RustTune) -> Theme {
    app.theme_choosen.clone()
}

// ====================== PLAYER ======================
impl RustTune {
   pub fn play_song(&mut self, file_path: &str) {
        // Arrêter l'ancienne lecture, si y'en a
        let _ = self.player.lock().unwrap().take();

        self.current_song = Some(file_path.to_string());
        self.is_playing = true;

        let player_clone = Arc::clone(&self.player);
        let stream_clone = Arc::clone(&self._stream);
        let path = file_path.to_string();

        std::thread::spawn(move || {
            let handle = rodio::DeviceSinkBuilder::open_default_sink()
                .expect("Impossible d'ouvrir le périphérique audio");

            {
                let mut s = stream_clone.lock().unwrap();
                *s = Some(handle);
            }

            let guard = stream_clone.lock().unwrap();
            let mixer = guard.as_ref().unwrap().mixer();
            let player = Player::connect_new(&mixer);

            // Lecture
            if let Ok(file) = File::open(&path) {
                let buffered = BufReader::new(file);
                if let Ok(source) = Decoder::new(buffered) {
                    player.append(source);
                    let mut p = player_clone.lock().unwrap();
                    *p = Some(player);
                }
            }
        });
    }
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