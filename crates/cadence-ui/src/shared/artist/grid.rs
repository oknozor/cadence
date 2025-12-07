use cadence_core::model::Artist;
use dioxus::prelude::*;

use crate::shared::ArtistCard;

#[component]
pub fn ArtistGrid(artists: Vec<Artist>, on_card_clicked: EventHandler<String>) -> Element {
    rsx! {
        div { class: "artist-grid",
            for artist in artists {
                ArtistCard { artist, on_card_clicked }
            }
        }
    }
}
