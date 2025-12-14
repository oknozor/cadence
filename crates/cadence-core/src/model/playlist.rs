use opensubsonic_cli::types::Playlist;

use crate::model::cover_url;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlaylistInfo {
    pub id: String,
    pub name: String,
    pub cover_art: Option<String>,
}

impl From<Playlist> for PlaylistInfo {
    fn from(value: Playlist) -> Self {
        PlaylistInfo {
            id: value.id,
            name: value.name,
            cover_art: value.cover_art.as_deref().map(cover_url),
        }
    }
}
