#[derive(Clone, PartialEq)]
pub struct Song {
    pub id: String,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub cover_art: Option<String>,
    pub track_number: Option<i64>,
    pub duration: Option<i64>,
}
