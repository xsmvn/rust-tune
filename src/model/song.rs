use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Song{
    pub id: u32,
    pub title: String,
    pub genre: String,
    pub release_year: u32,
    pub artiste: String,
}
