use cadence_core::model::{Album, Song};
use dioxus::prelude::*;

use crate::components::{SongItemInfo, TrackRow};

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

#[component]
pub fn TrackListWithCover(title: String, songs: Vec<Song>) -> Element {
    rsx! {
        div { class: "track-list-with-cover-container",
            h2 { "{title}" }

            div { class: "track-list",
                for song in songs {
                    SongItemInfo { song }
                }
            }
        }
    }
}
