use cadence_core::model::Album;
use dioxus::prelude::*;

use crate::track::TrackRow;

#[component]
pub fn TrackList(album: Album) -> Element {
    rsx! {
        div { class: "track-list",
            for track in album.songs {
                TrackRow { track }
            }
        }
    }
}
