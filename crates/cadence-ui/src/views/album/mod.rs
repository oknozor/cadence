use crate::album::AlbumMenuModal;
use crate::icons::download::DownloadIcon;
use crate::icons::plus::PlusIcon;
use crate::icons::share::ShareIcon;
use crate::modal::MenuModal;
use crate::scroller::VerticalScroller;
use crate::{
    album::{AlbumActionBar, AlbumCover, AlbumTitle},
    track::TrackList,
};
use cadence_core::hooks::use_album;
use dioxus::html::dialog::open;
use dioxus::prelude::*;

#[component]
pub fn AlbumView(id: String) -> Element {
    let album = use_album(id);
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
                    AlbumMenuModal {open: modal_open, album }
                }
            },
            None => rsx! {
                div { class: "loading", "Loading album..." }
            },
        }
    }
}
