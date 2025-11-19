use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Album {
    pub id: String,
    pub name: String,
    pub artist: String,
    pub year: Option<u16>,
    pub cover_art: Option<String>,
    pub songs: Vec<Song>,
}

#[derive(Clone, PartialEq)]
pub struct Song {
    pub id: String,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub cover_art: Option<String>,
    pub duration: Option<u32>,
}

impl From<opensubsonic_cli::types::AlbumId3WithSongs> for Album {
    fn from(response: opensubsonic_cli::types::AlbumId3WithSongs) -> Self {
        let artist = response.artist.unwrap_or_default();
        let album = response.name;

        Album {
            id: response.id,
            name: album.clone(),
            artist: artist.clone(),
            year: response.year.map(|y| y as u16),
            cover_art: response.cover_art,
            songs: response
                .song
                .into_iter()
                .map(|song| Song {
                    id: song.id,
                    title: song.title,
                    duration: song.duration.map(|d| d as u32),
                    artist: artist.clone(),
                    album: album.clone(),
                })
                .collect(),
        }
    }
}

impl From<opensubsonic_cli::types::AlbumId3> for Album {
    fn from(response: opensubsonic_cli::types::AlbumId3) -> Self {
        let artist = response.artist.unwrap_or_default();
        let album = response.name;

        Album {
            id: response.id,
            name: album.clone(),
            artist: artist.clone(),
            year: response.year.map(|y| y as u16),
            cover_art: response.cover_art,
            songs: response
                .song
                .into_iter()
                .map(|song| Song {
                    id: song.id,
                    title: song.title,
                    duration: song.duration.map(|d| d as u32),
                    artist: artist.clone(),
                    album: album.clone(),
                })
                .collect(),
        }
    }
}

#[component]
pub fn AlbumCard(album: Album, on_album_select: EventHandler<String>) -> Element {
    rsx! {
        div {
            class: "album-card",
            key: "{album.id}",
            onclick: move |_| on_album_select.call(album.id.clone()),

            if let Some(cover) = album.cover_art.as_ref() {
                img {
                    src: "{cover}",
                    alt: "{album.name}",
                    width: "100px",
                    height: "100px"
                }
            } else {
                div {
                    class: "no-cover",
                    "🎵"
                }
            }

            div {
                class: "album-info",
                span {
                    class: "album-name",
                    "{album.name}"
                }
                span {
                    class: "album-artist",
                    "{album.artist}"
                }
            }
        }
    }
}
