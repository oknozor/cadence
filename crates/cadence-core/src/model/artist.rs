use crate::model::Album;
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
    fn from(response: ArtistWithAlbumsId3) -> Self {
        let albums = response.album;
        let cover_art = response.cover_art;

        Artist {
            id: response.id,
            name: response.name,
            cover_art: cover_art.clone(),
            albums: albums.into_iter().map(Album::from).collect(),
            starred: response.starred.is_some(),
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
            cover_art: value.cover_art,
            albums: vec![],
            starred: value.starred.is_some(),
            bio: None,
            similar: vec![],
        }
    }
}
