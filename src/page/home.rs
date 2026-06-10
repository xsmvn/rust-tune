use iced::{Element, Length, widget::{column, text, container, row, scrollable}};
use iced_aw::Wrap;
use crate::model::song::Song;


#[derive(Clone, Debug)]
pub struct HomePage {
    pub Songs: Vec<Song>,
}

impl HomePage {
    pub fn new() -> Self {
        let songs: Vec<Song> = Self::load_songs();
        HomePage {Songs: songs}
    }


    
    pub fn load_songs() -> Vec<Song> {
        Vec::new()
    }

    
    pub fn view<'a>(&'a self) -> Element<'a,crate::Message> {
        let art_cover = self.Songs.iter()
            .map(|song| self.create_song_card(song))
            .collect::<Vec<_>>();

        column![
            text("Home").size(28),
            text("Songs : ").size(20),
            scrollable(
                Wrap::with_elements(art_cover)
                .spacing(16.0)
                .line_spacing(16.0)
            ),
        ]
        .spacing(25)
        .padding(0)
        .into()
    }

    fn create_song_card<'a>(&'a self, song: &crate::model::song::Song) -> Element<'a,crate::Message> {
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
    .into()
}
}