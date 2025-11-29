use crate::icons::{
    download::DownloadIcon, play::PlayIcon, plus::PlusIcon, share::ShareIcon, shuffle::ShuffleIcon,
};
use cadence_core::state::{ControllerExt, ControllerStoreExt};
use cadence_core::{model::Song, state::CONTROLLER};
use dioxus::prelude::*;

#[component]
pub fn AlbumActionBar(songs: Vec<Song>) -> Element {
    rsx! {
        div { class: "album-action-bar",
            div { class: "album-action-bar-start",
                button {
                    DownloadIcon { size: 32 }
                }

                button {
                    PlusIcon { size: 32, filled: false }
                }

                button {
                    ShareIcon { size: 32 }
                }
            }

            div { class: "album-action-bar-end",
                button {
                    ShuffleIcon { size: 36, filled: CONTROLLER.resolve().shuffle() }
                }

                button {
                    onclick: move |_| {
                        if *CONTROLLER.resolve().is_playing().read() {
                            CONTROLLER.resolve().toggle_play();
                        } else {
                            CONTROLLER.resolve().queue_all(songs.clone());
                        }
                    },
                    PlayIcon { size: 48, is_playing: CONTROLLER.resolve().is_playing() }
                }
            }
        }
    }
}
