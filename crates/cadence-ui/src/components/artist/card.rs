use cadence_core::model::Artist;
use dioxus::prelude::*;

use crate::components::RoundedThumbnail;

#[component]
pub fn ArtistCard(artist: Artist, on_card_clicked: EventHandler<String>) -> Element {
    rsx! {
        div {
            class: "artist-card",
            key: "{artist.id}",
            onclick: move |_| on_card_clicked.call(artist.id.clone()),
            if let Some(src) = artist.cover_art {
                RoundedThumbnail { src, name: artist.name.clone(), size: 96 }
            } else {
                div {
                    class: "artist-placeholder",
                    style: "width: 96px; height: 96px; border-radius: 50%; background: #ccc; display: flex; align-items: center; justify-content: center;",
                    "{artist.name.chars().next().unwrap_or_default()}"
                }
            }
            span { class: "artist-name", "{artist.name}" }
        }
    }
}
