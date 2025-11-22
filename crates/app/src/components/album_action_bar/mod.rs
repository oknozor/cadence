use crate::context::IsPlaying;
use cadence_ui::icons::{play::PlayIcon, shuffle::ShuffleIcon};
use dioxus::prelude::*;

#[component]
pub fn AlbumActionBar() -> Element {
    let playing: IsPlaying = use_context();

    rsx! {
        div {
            class: "album-action-bar",
            div {
                class: "album-action-bar-start",
                div {
                    button {
                        "Add to Queue"
                    }
                }

                div {
                    button {
                        "Share"
                    }
                }
            }

            div {
                class: "album-action-bar-end",
                div {
                    button {
                        class: "album-action-bar-button",
                        ShuffleIcon { size: 36, filled: playing.is_shuffle() }
                    }

                    button {
                        class: "album-action-bar-button",
                        PlayIcon { size: 48, is_playing: playing.is_playing() }
                    }
                }
            }
        }
    }
}
