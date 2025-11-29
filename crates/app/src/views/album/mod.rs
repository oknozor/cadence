use cadence_core::hooks::use_album;
use cadence_ui::{
    album::{AlbumActionBar, AlbumCover, AlbumTitle},
    track::TrackList,
};
use dioxus::prelude::*;

#[component]
pub fn AlbumView(id: String) -> Element {
    let album = use_album(id);

    rsx! {
        div { class: "album-view",
            match album() {
                Some(album) => rsx! {
                    AlbumCover { src: album.cover_art.clone().unwrap_or_default() }
                    div { class: "album-info view",
                        AlbumTitle {
                            name: album.name.clone(),
                            artist: album.artist.clone(),
                            year: album.year.clone(),
                        }
                        AlbumActionBar { songs: album.songs.clone() }
                        TrackList { album }
                    }
                },
                None => rsx! {
                    div { class: "loading", "Loading album..." }
                },
            }
        }
    }
}
