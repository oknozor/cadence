use opensubsonic_cli::types::{AlbumId3, ArtistId3, Child};

use crate::model::{Song, cover_url, ensure_https};

#[derive(Debug, Clone, PartialEq)]
pub enum SearchResult {
    Artist {
        id: String,
        name: String,
        thumbnail: Option<String>,
    },
    Album {
        id: String,
        name: String,
        cover: Option<String>,
        artist: Option<String>,
    },
    Song(Song),
}

impl From<AlbumId3> for SearchResult {
    fn from(album: AlbumId3) -> Self {
        SearchResult::Album {
            id: album.id,
            name: album.name,
            cover: album.cover_art.as_deref().map(cover_url),
            artist: album.display_artist.or(album.artist),
        }
    }
}

impl From<ArtistId3> for SearchResult {
    fn from(artist: ArtistId3) -> Self {
        SearchResult::Artist {
            id: artist.id,
            name: artist.name,
            thumbnail: artist.artist_image_url.map(ensure_https),
        }
    }
}

impl From<Child> for SearchResult {
    fn from(song: Child) -> Self {
        SearchResult::Song(Song {
            id: song.id,
            title: song.title,
            artist: song
                .display_artist
                .or(song.artist)
                .unwrap_or("Unkown artist".to_string()),
            album: song.album.unwrap_or("Unkown album".to_string()),
            cover_art: song.cover_art.as_deref().map(cover_url),
            track_number: song.disc_number,
            duration: song.duration,
            artist_id: song.artist_id,
            album_id: song.album_id,
            starred: song.starred.is_some(),
        })
    }
}
