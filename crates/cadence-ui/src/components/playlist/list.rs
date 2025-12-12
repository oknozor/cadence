use cadence_core::model::PlaylistInfo;
use dioxus::prelude::*;

use crate::components::{HorizontalScroller, PlaylistCard};

#[component]
pub fn PlaylistList(
    title: String,
    playlists: Vec<PlaylistInfo>,
    on_card_clicked: EventHandler<String>,
) -> Element {
    rsx! {
        div { class: "playlist-container",
            h2 { "{title}" }

            HorizontalScroller {
                for playlist in playlists {
                    PlaylistCard {
                        playlist,
                        on_card_clicked: on_card_clicked.clone(),
                    }
                }
            }
        }
    }
}
