use iced::{Element, Length, widget::{column, text, container, row, scrollable, button}, Alignment};
use iced_aw::Wrap;
use crate::model::song::Song;
use std::fs;
use std::path::{Path, PathBuf};
use crate::Message;
use rfd::FileDialog;

#[derive(Clone, Debug)]
pub struct HomePage {
    pub Songs: Vec<Song>,
}

impl HomePage {
    pub fn new() -> Self {
        // Crée le dossier au démarrage si besoin
        let _ = Self::music_dir();
        HomePage { Songs: Self::load_songs() }
    }

    /// Retourne le chemin du dossier musique de l'utilisateur
    /// et le crée automatiquement s'il n'existe pas.
    fn music_dir() -> PathBuf {
        let dir = dirs::data_dir()
            .unwrap_or_else(|| dirs::home_dir().unwrap_or_else(|| PathBuf::from(".")))
            .join("rust-tune")
            .join("music");

        if !dir.exists() {
            if let Err(e) = fs::create_dir_all(&dir) {
                eprintln!("Impossible de créer le dossier musique : {}", e);
            } else {
                println!("Dossier musique créé : {}", dir.display());
            }
        }

        dir
    }

    pub fn load_songs() -> Vec<Song> {
        let mut songs = Vec::new();
        let music_dir = Self::music_dir();

        if let Ok(entries) = fs::read_dir(&music_dir) {
            for entry in entries.filter_map(Result::ok) {
                let path = entry.path();
                if let Some(ext) = path.extension() {
                    let ext = ext.to_string_lossy().to_lowercase();
                    if ["mp3", "wav", "flac", "ogg"].contains(&ext.as_str()) {
                        songs.push(Song::from_path(&path, songs.len() as u32 + 1));
                    }
                }
            }
        }

        songs
    }

    pub fn refresh(&mut self) {
        self.Songs = Self::load_songs();
        println!("Bibliothèque rafraîchie : {} musiques", self.Songs.len());
    }

    pub fn view<'a>(&'a self, theme: &'a iced::Theme) -> Element<'a, crate::Message> {
        let art_cover = self.Songs.iter()
            .map(|song| self.create_song_card(song, theme))
            .collect::<Vec<_>>();

        column![
           row![
                text("Home").size(28),
                iced::widget::Space::new().width(Length::Fill),
                button(text("Rafraîchir"))
                    .style(crate::transparent_button_style(theme.clone()))
                    .on_press(Message::RefreshLibrary),
                button(text("Ajouter une chanson"))
                    .style(crate::transparent_button_style(theme.clone()))
                    .on_press(Message::AddSong),
            ]
            .spacing(15)
            .align_y(Alignment::Center),

            text(format!("Songs : {}", self.Songs.len())).size(20),

            scrollable(
                Wrap::with_elements(art_cover)
                    .spacing(16.0)
                    .line_spacing(16.0)
            ).height(Length::Fill),
        ]
        .spacing(25)
        .padding(20)
        .into()
    }

    fn create_song_card<'a>(&'a self, song: &Song, theme: &'a iced::Theme) -> Element<'a, crate::Message> {
        let file_name = Path::new(&song.file_path)
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| song.file_path.clone());

        let title_row = row![
            text(song.title.clone())
                .size(18)
                .width(Length::Fill)
                .style(move |_t| {
                    iced::widget::text::Style {
                        color: match theme {
                            iced::Theme::Light => Some(iced_aw::style::colors::DARK),
                            _ => Some(iced_aw::style::colors::WHITE),
                        }
                    }
                }),

            button(text("🗑").size(16))
                .style(crate::transparent_button_style(theme.clone()))
                .on_press(Message::DeleteSong(song.file_path.clone()))
                .padding([4, 8]),
        ]
        .spacing(8)
        .align_y(Alignment::Center);

        let content = button(
            container(
                column![
                    title_row,
                    text(song.artiste.clone())
                        .size(14)
                        .style(move |_t| {
                            iced::widget::text::Style {
                                color: match theme {
                                    iced::Theme::Light => Some(iced::Color::from_rgb(0.3, 0.3, 0.3)),
                                    _ => Some(iced::Color::from_rgb(0.7, 0.7, 0.7)),
                                }
                            }
                        }),
                    text(file_name)
                        .size(11)
                        .style(move |_t| {
                            iced::widget::text::Style {
                                color: match theme {
                                    iced::Theme::Light => Some(iced::Color::from_rgb(0.5, 0.5, 0.5)),
                                    _ => Some(iced::Color::from_rgb(0.55, 0.55, 0.55)),
                                }
                            }
                        }),
                ]
                .spacing(4)
                .padding(12)
            )
            .width(Length::Fixed(340.0))
        )
        .style(crate::transparent_button_style(theme.clone()))
        .on_press(Message::PlaySong(song.file_path.clone()));

        content.into()
    }

    pub fn delete_song(&mut self, path: &str) {
        let file_path = Path::new(path);

        match fs::remove_file(file_path) {
            Ok(_) => {
                println!("Fichier supprimé : {}", path);
                self.refresh();
            }
            Err(e) => {
                eprintln!("Erreur lors de la suppression de {} : {}", path, e);
            }
        }
    }

    // directory of the files should be /home/username/.local/share/rust-tune/music
    pub fn add_song(&mut self) {
        if let Some(file_path) = FileDialog::new()
            .add_filter("Audio", &["mp3", "wav", "flac", "ogg"])
            .set_title("Choisir un fichier audio")
            .pick_file()
        {
            let dest_dir = Self::music_dir();

            if let Some(file_name) = file_path.file_name() {
                let dest_path = dest_dir.join(file_name);

                // Évite d’écraser un fichier existant (optionnel)
                let dest_path = if dest_path.exists() {
                    let stem = file_path.file_stem().unwrap_or_default().to_string_lossy();
                    let ext = file_path.extension().unwrap_or_default().to_string_lossy();
                    dest_dir.join(format!("{}_copie.{}", stem, ext))
                } else {
                    dest_path
                };

                match fs::copy(&file_path, &dest_path) {
                    Ok(_) => {
                        println!("Fichier ajouté : {}", dest_path.display());
                        self.refresh();
                    }
                    Err(e) => eprintln!("Erreur lors de la copie : {}", e),
                }
            }
        }
    }
}