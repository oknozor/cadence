use cadence_core::model::Album;
use dioxus::prelude::*;

use crate::album::card::AlbumCard;
use crate::scroller::HorizontalScroller;

#[component]
pub fn AlbumList(
    title: String,
    albums: Vec<Album>,
    on_album_select: EventHandler<String>,
) -> Element {
    rsx! {
        div { class: "tracklist-container",
            h2 { "{title}" }

            HorizontalScroller {
                for album in albums {
                    AlbumCard { album, on_album_select }
                }
            }
        }
    }
}
