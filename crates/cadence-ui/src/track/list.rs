use cadence_core::model::Album;
use dioxus::prelude::*;

use crate::track::TrackRow;

#[component]
pub fn TrackList(album: Album) -> Element {
    rsx! {
        div { class: "track-list",
            for song in album.songs {
                TrackRow { song }
            }
        }
    }
}
