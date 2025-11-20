use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Album {
    pub id: String,
    pub name: String,
    pub artist: String,
    pub year: Option<i64>,
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
    pub track_number: Option<i64>,
    pub duration: Option<i64>,
}

impl From<opensubsonic_cli::types::AlbumId3WithSongs> for Album {
    fn from(response: opensubsonic_cli::types::AlbumId3WithSongs) -> Self {
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

impl From<opensubsonic_cli::types::AlbumId3> for Album {
    fn from(response: opensubsonic_cli::types::AlbumId3) -> Self {
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
