use iced::{Element, futures::future::Map, widget::{column, container, text}};
use crate::page::home::HomePage;
use crate::model::song::Song;

#[derive(Clone, Debug)]
pub struct ProfilePage;

impl ProfilePage {
    pub fn new() -> Self {
        ProfilePage {}
    }

   /*     fn games_completion_ratio(&self, games: Vec<Game>) -> Map<String, u32> {
            let stats: Map<String,u32>;
            stats
        }
    */


    // ✨ Nouvelle méthode: affichage complet
    pub fn view<'a>(&'a self, home_page: &HomePage) -> Element<'a,crate::Message> {
        let games = &home_page;

        column![
            text("Profil").size(24),
            text(format!("Musica Musica ++++")),

            container(
                text("Vos statistique"),
                //games_completion_ratio(games)
            ),

        ]
        .spacing(20)
        .into()
    }




}