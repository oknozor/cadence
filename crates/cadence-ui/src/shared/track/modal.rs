use crate::{
    shared::MenuModal,
    shared::{ArtistIcon, DownloadIcon, ItemInfo, PlusIcon, ShareIcon, Thumbnail},
    views::Route,
};
use cadence_core::model::Song;
use dioxus::prelude::*;

#[component]
pub fn TrackMenuModal(open: Signal<bool>, song: Song) -> Element {
    let nav = navigator();
    let thumbnail = song.cover_art.map(|src| {
        rsx! {
            Thumbnail { size: 42, name: song.title.clone(), src }
        }
    });

    rsx! {
        MenuModal { open,
            div { class: "modal-headers",
                if let Some(thumbnail) = thumbnail {
                    {thumbnail}
                }
                ItemInfo { primary: song.title.clone(), secondary: song.album }
            }
            div { class: "modal-menu-item", onclick: move |_| {},
                ShareIcon { size: 24 }
                span { "Share" }
            }
            div { class: "modal-menu-item", onclick: move |_| {},
                PlusIcon { size: 24, filled: false }
                span { "Add to liked songs" }
            }
            div { class: "modal-menu-item", onclick: move |_| {},
                DownloadIcon { size: 24 }
                span { "Download" }
            }
            if let Some(artist_id) = song.artist_id {
                div {
                    class: "modal-menu-item",
                    onclick: move |_| {
                        nav.push(Route::ArtistView {
                            id: artist_id.clone(),
                        });
                    },
                    ArtistIcon { size: 24 }
                    span { "Go to artist" }
                }
            }
            div { class: "modal-menu-item", onclick: move |_| {},
                PlusIcon { size: 24, filled: false }
                span { "Add to playlist" }
            }
        }
    }
}
