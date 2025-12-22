use opensubsonic_cli::types::Child;

use crate::model::cover_url;

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
    pub starred: bool,
}

impl From<Child> for Song {
    fn from(value: Child) -> Self {
        Song {
            id: value.id,
            title: value.title,
            artist: value
                .display_artist
                .or(value.artist)
                .unwrap_or("Unkown artist".to_string()),
            album: value.album.unwrap_or("Unkown album".to_string()),
            cover_art: value.cover_art.as_deref().map(cover_url),
            track_number: value.disc_number,
            duration: value.duration,
            artist_id: value.artist_id,
            album_id: value.album_id,
            starred: value.starred.is_some(),
        }
    }
}
