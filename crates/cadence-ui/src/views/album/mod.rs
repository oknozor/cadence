use crate::shared::VerticalScroller;
use crate::shared::{AlbumActionBar, AlbumCover};
use crate::track::TrackList;
use cadence_core::hooks::use_album;
use components::{AlbumMenuModal, AlbumTitle};
use dioxus::prelude::*;

mod components;

#[component]
pub fn AlbumView(id: ReadSignal<String>) -> Element {
    let current_id = use_signal(|| id());
    let album = use_album(current_id);
    let modal_open = use_signal(|| false);

    rsx! {
        match album() {
            Some(album) => rsx! {
                div { class: "album-info",
                    AlbumCover { src: album.cover_art.clone().unwrap_or_default(), width: "200px" }
                    AlbumTitle {
                        name: album.name.clone(),
                        artist: album.artist.clone(),
                        year: album.year,
                    }
                    AlbumActionBar { songs: album.songs.clone(), modal_open }
                    VerticalScroller {
                        TrackList { album: album.clone() }
                    }
                    AlbumMenuModal { open: modal_open, album }
                }
            },
            None => rsx! {
                div { class: "loading", "Loading album..." }
            },
        }
    }
}
