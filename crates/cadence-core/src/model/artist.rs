use crate::model::{Album, cover_url};
use opensubsonic_cli::types;
use opensubsonic_cli::types::ArtistId3;
use types::ArtistWithAlbumsId3;

#[derive(Clone, PartialEq)]
pub struct Artist {
    pub id: String,
    pub name: String,
    pub cover_art: Option<String>,
    pub albums: Vec<Album>,
    pub starred: bool,
    pub bio: Option<String>,
    pub similar: Vec<Artist>,
}

impl From<ArtistWithAlbumsId3> for Artist {
    fn from(value: ArtistWithAlbumsId3) -> Self {
        let albums = value.album;
        let cover_art = value.cover_art.as_deref().map(cover_url);

        Artist {
            id: value.id,
            name: value.name,
            cover_art: cover_art.clone(),
            albums: albums.into_iter().map(Album::from).collect(),
            starred: value.starred.is_some(),
            bio: None,
            similar: vec![],
        }
    }
}

impl From<ArtistId3> for Artist {
    fn from(value: ArtistId3) -> Self {
        Artist {
            id: value.id,
            name: value.name,
            cover_art: value.cover_art.as_deref().map(cover_url),
            albums: vec![],
            starred: value.starred.is_some(),
            bio: None,
            similar: vec![],
        }
    }
}
