use crate::model::{cover_url, song::Song};
use opensubsonic_cli::types::{AlbumId3, AlbumId3WithSongs};

#[derive(Clone, PartialEq)]
pub struct Album {
    pub id: String,
    pub name: String,
    pub artist: String,
    pub artist_id: String,
    pub year: Option<i64>,
    pub cover_art: Option<String>,
    pub songs: Vec<Song>,
    pub starred: bool,
}

impl From<AlbumId3WithSongs> for Album {
    fn from(response: AlbumId3WithSongs) -> Self {
        let artist = response.artist.unwrap_or_default();
        let album = response.name;
        let cover_art = response.cover_art.as_deref().map(cover_url);
        let artist_id = response.artist_id.unwrap_or_default();

        Album {
            id: response.id.clone(),
            name: album.clone(),
            artist: artist.clone(),
            artist_id: artist_id.clone(),
            year: response.year,
            cover_art: cover_art.clone(),
            songs: response
                .song
                .into_iter()
                .map(|song| Song {
                    id: song.id,
                    track_number: song.track,
                    title: song.title,
                    duration: song.duration,
                    artist: artist.clone(),
                    cover_art: cover_art.clone(),
                    album: album.clone(),
                    artist_id: Some(artist_id.clone()),
                    album_id: Some(response.id.clone()),
                    starred: song.starred.is_some(),
                })
                .collect(),
            starred: response.starred.is_some(),
        }
    }
}

impl From<AlbumId3> for Album {
    fn from(response: AlbumId3) -> Self {
        // FIXME: optionals should not default
        let id = response.id;
        let artist = response.artist.unwrap_or_default();
        let artist_id = response.artist_id.unwrap_or_default();
        let album = response.name;
        let cover_art = response.cover_art.as_deref().map(cover_url);

        Album {
            id: id.clone(),
            name: album.clone(),
            artist: artist.clone(),
            artist_id: artist_id.clone(),
            year: response.year,
            cover_art: cover_art.clone(),
            starred: response.starred.is_some(),
            songs: response
                .song
                .into_iter()
                .map(|song| Song {
                    id: song.id,
                    title: song.title,
                    track_number: song.track,
                    duration: song.duration,
                    artist: artist.clone(),
                    cover_art: cover_art.clone(),
                    album: album.clone(),
                    artist_id: Some(artist_id.clone()),
                    album_id: Some(id.clone()),
                    starred: song.starred.is_some(),
                })
                .collect(),
        }
    }
}
