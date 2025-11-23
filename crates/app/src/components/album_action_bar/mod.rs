use cadence_core::hooks::use_player_state;
use cadence_ui::icons::{play::PlayIcon, shuffle::ShuffleIcon};
use dioxus::prelude::*;

#[component]
pub fn AlbumActionBar() -> Element {
    let player = use_player_state();

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
                        ShuffleIcon { size: 36, filled: player.is_shuffle() }
                    }

                    button {
                        class: "album-action-bar-button",
                        PlayIcon { size: 48, is_playing: player.is_playing() }
                    }
                }
            }
        }
    }
}
