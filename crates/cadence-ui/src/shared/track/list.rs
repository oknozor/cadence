use cadence_core::model::{Album, Song};
use dioxus::prelude::*;

use crate::shared::TrackRow;

#[component]
pub fn TrackList(album: Album, action_clicked: EventHandler<Song>) -> Element {
    rsx! {
        div { class: "track-list",
            for song in album.songs {
                TrackRow { song, action_clicked }
            }
        }
    }
}
