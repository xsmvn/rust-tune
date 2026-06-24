use iced::{Element, Length, widget::{column, text, container, row, scrollable, button}};
use iced_aw::Wrap;
use crate::model::song::Song;
use std::fs;
use std::path::Path;
use crate::Message;

#[derive(Clone, Debug)]
pub struct HomePage {
    pub Songs: Vec<Song>,
}

impl HomePage {
    pub fn new() -> Self {
        let songs: Vec<Song> = Self::load_songs();
        HomePage { Songs: songs }
    }

    pub fn load_songs() -> Vec<Song> {
        let mut songs = Vec::new();
        let music_dir = Path::new("music_files");

        if !music_dir.exists() {
            println!("Dossier 'music_files' non trouvé.");
            return songs;
        }

        if let Ok(entries) = fs::read_dir(music_dir) {
            for entry in entries.filter_map(Result::ok) {
                let path = entry.path();
                if let Some(ext) = path.extension() {
                    if ext == "mp3" || ext == "wav" || ext == "flac" || ext == "ogg" {
                        let file_name = path.file_name().unwrap().to_string_lossy().to_string();
                        let title = file_name.replace(".mp3", "").replace(".wav", "")
                                           .replace(".flac", "").replace(".ogg", "");

                        let song = Song {
                            id: songs.len() as u32 + 1,
                            title: title.clone(),
                            genre: "Unknown".to_string(),
                            release_year: 2025,
                            artiste: "Unknown Artist".to_string(),
                            file_path: path.to_string_lossy().to_string(),
                        };
                        songs.push(song);
                    }
                }
            }
        }
        songs
    }

    pub fn view<'a>(&'a self) -> Element<'a, crate::Message> {
        let art_cover = self.Songs.iter()
            .map(|song| self.create_song_card(song))
            .collect::<Vec<_>>();

        column![
            text("Home").size(28),
            text(format!("Songs : {}", self.Songs.len())).size(20),
            scrollable(
                Wrap::with_elements(art_cover)
                    .spacing(16.0)
                    .line_spacing(16.0)
            ).height(Length::Fill),
        ]
        .spacing(25)
        .padding(0)
        .into()
    }

        fn create_song_card<'a>(&'a self, song: &crate::model::song::Song) -> Element<'a, crate::Message> {
        button(
            container(
                column![
                    text(song.title.clone()).size(19).width(Length::Fill),
                    row![
                        text(song.genre.clone()).size(14),
                        text(" • ").size(14),
                        text(song.artiste.clone()).size(14),
                    ]
                    .spacing(6),
                    text(format!("Artiste : {}", song.artiste.clone())).size(14),
                ]
                .spacing(10)
                .padding(10)
            )
            .width(Length::Fixed(340.0))
            .padding(0)
        )
        .style(crate::transparent_button_style(iced::Theme::Light))
        .on_press(Message::PlaySong(song.file_path.clone()))
        .into()
    }
}