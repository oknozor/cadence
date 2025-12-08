#[derive(Debug, Clone, PartialEq)]
pub struct Song {
    pub id: String,
    pub title: String,
    pub artist: String,
    pub artist_id: Option<String>,
    pub album: String,
    pub album_id: Option<String>,
    pub cover_art: Option<String>,
    pub track_number: Option<i64>,
    pub duration: Option<i64>,
}
