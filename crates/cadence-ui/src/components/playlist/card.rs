use cadence_core::model::PlaylistInfo;
use dioxus::prelude::*;

use crate::components::Thumbnail;

#[component]
pub fn PlaylistCard(playlist: PlaylistInfo, on_card_clicked: EventHandler<String>) -> Element {
    rsx! {
        div {
            class: "playlist-card",
            key: "{playlist.id}",
            onclick: move |_| on_card_clicked.call(playlist.id.clone()),
            if let Some(src) = playlist.cover_art {
                Thumbnail { src, name: playlist.name.clone(), size: 96 }
            } else {
                div {
                    class: "playlist-placeholder",
                    "{playlist.name.chars().next().unwrap_or_default()}"
                }
            }
            span { class: "playlist-name", "{playlist.name}" }
        }
    }
}
