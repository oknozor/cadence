//! Music library browser component

use dioxus::prelude::*;

use crate::components::album_card::{Album, AlbumCard};

#[component]
pub fn AlbumList(
    title: String,
    albums: Vec<Album>,
    on_album_select: EventHandler<String>,
) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div {
            class: "library-container",
            h2 { "{title}" }

            div {
                class: "albums-grid",
                for album in albums {
                   AlbumCard { album, on_album_select }
                }
            }
        }
    }
}
