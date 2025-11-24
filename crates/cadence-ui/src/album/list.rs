use cadence_core::model::Album;
use dioxus::prelude::*;

use crate::album::card::AlbumCard;

#[component]
pub fn AlbumList(
    title: String,
    albums: Vec<Album>,
    on_album_select: EventHandler<String>,
) -> Element {
    rsx! {
        div { class: "library-container",
            h2 { "{title}" }

            div { class: "albums-grid",
                for album in albums {
                    AlbumCard { album, on_album_select }
                }
            }
        }
    }
}
