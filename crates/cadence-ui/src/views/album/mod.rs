use crate::scroller::VerticalScroller;
use crate::{
    album::{AlbumActionBar, AlbumCover, AlbumTitle},
    track::TrackList,
};
use cadence_core::hooks::use_album;
use dioxus::html::sup::width;
use dioxus::prelude::*;

#[component]
pub fn AlbumView(id: String) -> Element {
    let album = use_album(id);

    rsx! {
        match album() {
            Some(album) => rsx! {
                AlbumCover { src: album.cover_art.clone().unwrap_or_default(), width: "200px" }
                div { class: "album-info",
                    AlbumTitle {
                        name: album.name.clone(),
                        artist: album.artist.clone(),
                        year: album.year,
                    }
                    AlbumActionBar { songs: album.songs.clone() }
                    VerticalScroller {
                        TrackList { album }
                    }
                }
            },
            None => rsx! {
                div { class: "loading", "Loading album..." }
            },
        }
    }
}
