use crate::{
    shared::MenuModal,
    shared::{ArtistIcon, DownloadIcon, ItemInfo, PlusIcon, ShareIcon, Thumbnail},
    views::Route,
};
use cadence_core::model::Album;
use dioxus::prelude::*;

#[component]
pub fn AlbumMenuModal(open: Signal<bool>, album: Album) -> Element {
    let nav = navigator();
    let thumbnail = album.cover_art.map(|src| {
        rsx! {
            Thumbnail { size: 42, name: album.name.clone(), src }
        }
    });

    rsx! {
        MenuModal { open,
            div { class: "modal-headers",
                if let Some(thumbnail) = thumbnail {
                    {thumbnail}
                }
                ItemInfo { primary: album.name, secondary: album.artist }
            }
            div { class: "modal-menu-item", onclick: move |_| {},
                ShareIcon { size: 24 }
                span { "Share" }
            }
            div { class: "modal-menu-item", onclick: move |_| {},
                PlusIcon { size: 24, filled: false }
                span { "Add to liked albums" }
            }
            div { class: "modal-menu-item", onclick: move |_| {},
                DownloadIcon { size: 24 }
                span { "Download" }
            }
            div {
                class: "modal-menu-item",
                onclick: move |_| {
                    nav.push(Route::ArtistView {
                        id: album.artist_id.clone(),
                    });
                },
                ArtistIcon { size: 24 }
                span { "Go to artist" }
            }
            div { class: "modal-menu-item", onclick: move |_| {},
                PlusIcon { size: 24, filled: false }
                span { "Add to playlist" }
            }
        }
    }
}
