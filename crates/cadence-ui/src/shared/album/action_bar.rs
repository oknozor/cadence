use crate::shared::{DotIcon, DownloadIcon, PlayIconCircle, PlusIcon, ShareIcon, ShuffleIcon};
use cadence_core::state::{ControllerExt, ControllerStoreExt};
use cadence_core::{model::Song, state::CONTROLLER};
use dioxus::prelude::*;

#[component]
pub fn AlbumActionBar(songs: Vec<Song>, modal_open: Signal<bool>) -> Element {
    let mut controller = CONTROLLER.resolve();

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

                button { onclick: move |_| modal_open.toggle(),
                    DotIcon { size: 32 }
                }
            }

            div { class: "album-action-bar-end",
                button {
                    ShuffleIcon { size: 36, filled: controller.shuffle() }
                }

                button {
                    onclick: move |_| {
                        if *controller.is_playing().read() {
                            controller.toggle_play();
                        } else {
                            controller.queue_all(songs.clone());
                        }
                    },
                    PlayIconCircle { size: 48, is_playing: controller.is_playing() }
                }
            }
        }
    }
}
