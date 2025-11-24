use crate::icons::{
    download::DownloadIcon, play::PlayIcon, plus::PlusIcon, share::ShareIcon, shuffle::ShuffleIcon,
};
use cadence_core::hooks::use_player_state;
use dioxus::prelude::*;

#[component]
pub fn AlbumActionBar() -> Element {
    let player = use_player_state();

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
                    ShuffleIcon { size: 36, filled: player.is_shuffle() }
                }

                button {
                    PlayIcon { size: 48, is_playing: player.is_playing() }
                }
            }
        }
    }
}
