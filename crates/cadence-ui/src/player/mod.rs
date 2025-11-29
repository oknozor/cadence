use crate::icons::play::PlayIcon;
use crate::items::ItemInfo;
use crate::progress::Progress;
use crate::progress::ProgressIndicator;
use crate::thumbnails::Thumbnail;
use cadence_core::hooks::use_playback_position;
use cadence_core::state::{CONTROLLER, ControllerExt, ControllerStoreExt};
use dioxus::prelude::*;

#[component]
pub fn Player() -> Element {
    let playback_position = use_playback_position();
    let mut controller = CONTROLLER.resolve();

    rsx! {
        div { class: "player-container",
            if let Some(track) = controller.current() {
                div {
                    display: "flex",
                    flex_direction: "row",
                    align_items: "center",
                    padding_left: "10px",
                    div {
                        class: "track-container",
                        flex: "column",
                        flex_grow: 1,
                        if let Some(cover) = track.cover_art.as_ref() {
                            Thumbnail { src: cover, name: &track.title, size: 32 }
                        }
                        ItemInfo { primary: track.title, secondary: track.artist }
                    }
                    div { class: "player-controls",
                        button {
                            onclick: move |_| {
                                controller.toggle_play();
                            },
                            PlayIcon { size: 24, is_playing: controller.is_playing() }
                        }
                    }
                }
                Progress {
                    value: playback_position,
                    max: track.duration.unwrap_or_default() as f64,
                    ProgressIndicator {}
                }
            } else {
                div {
                }
            }
        }
    }
}
