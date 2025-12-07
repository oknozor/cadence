use cadence_core::model::Album;
use dioxus::prelude::*;

use super::card::AlbumCard;
use crate::shared::HorizontalScroller;

#[component]
pub fn AlbumList(
    title: String,
    albums: Vec<Album>,
    on_card_clicked: EventHandler<String>,
) -> Element {
    rsx! {
        div { class: "tracklist-container",
            h2 { "{title}" }

            HorizontalScroller {
                for album in albums {
                    AlbumCard { album, on_card_clicked }
                }
            }
        }
    }
}
