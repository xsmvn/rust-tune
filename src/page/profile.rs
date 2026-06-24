use iced::{Element, futures::future::Map, widget::{column, container, text}};
use crate::page::home::HomePage;
use crate::model::song::Song;

#[derive(Clone, Debug)]
pub struct ProfilePage;

impl ProfilePage {
    pub fn new() -> Self {
        ProfilePage {}
    }


    pub fn view<'a>(&'a self, home_page: &HomePage) -> Element<'a,crate::Message> {
        let songs = &home_page;

        column![
            text("Profil").size(24),
            text(format!("Musica Musica ++++")),

            container(
                text("Vos statistique"),
            ),

        ]
        .spacing(20)
        .into()
    }




}