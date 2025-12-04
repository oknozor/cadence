use crate::model::song::Song;
use opensubsonic_cli::types::{AlbumId3, AlbumId3WithSongs};

#[derive(Clone, PartialEq)]
pub struct Album {
    pub id: String,
    pub name: String,
    pub artist: String,
    pub year: Option<i64>,
    pub cover_art: Option<String>,
    pub songs: Vec<Song>,
}

impl From<AlbumId3WithSongs> for Album {
    fn from(response: AlbumId3WithSongs) -> Self {
        let artist = response.artist.unwrap_or_default();
        let album = response.name;
        let cover_art = response.cover_art;

        Album {
            id: response.id,
            name: album.clone(),
            artist: artist.clone(),
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
                })
                .collect(),
        }
    }
}

impl From<AlbumId3> for Album {
    fn from(response: AlbumId3) -> Self {
        let artist = response.artist.unwrap_or_default();
        let album = response.name;
        let cover_art = response.cover_art;

        Album {
            id: response.id,
            name: album.clone(),
            artist: artist.clone(),
            year: response.year,
            cover_art: cover_art.clone(),
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
                })
                .collect(),
        }
    }
}
