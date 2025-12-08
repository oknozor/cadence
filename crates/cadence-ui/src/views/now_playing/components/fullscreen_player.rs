use crate::shared::{
    AlbumCover, ItemInfo, NextIcon, PlayIconCircle, PlayerProgress, PlusIcon, PreviousIcon,
    RandomIcon, ShuffleIcon,
};
use cadence_core::state::{CONTROLLER, ControllerExt, ControllerStoreExt};
use dioxus::prelude::*;

#[component]
pub fn FullScreenPlayer() -> Element {
    let mut controller = CONTROLLER.resolve();
    let current = controller.current();
    let is_playing = controller.is_playing()();
    let is_shuffle = controller.shuffle()();
    let is_random = controller.random()();
    let has_next = use_memo(move || {
        let current = *controller.current_idx().read();
        let last = controller.queue_store().read().len();
        current != last
    });

    let has_previous = use_memo(move || {
        let current = *controller.current_idx().read();
        current == 0
    });

    let track_duration = current
        .map(|song| {
            let duration = song.read().1.duration.unwrap_or_default();
            let minutes = duration / 60;
            let seconds = duration % 60;
            format!("{:02}:{:02}", minutes, seconds)
        })
        .unwrap_or("00:00".to_string());

    rsx! {
        div { class: "player-fullscreen",
            div { class: "controls-container",
                if let Some(track) = current {
                    if let Some(src) = track.read().1.cover_art.clone() {
                        AlbumCover { src, width: "100%" }
                    }
                    div { class: "track-info",
                        ItemInfo {
                            primary: track.read().1.title.clone(),
                            secondary: track.read().1.artist.clone(),
                        }
                        PlusIcon { filled: false }
                    }

                    PlayerProgress {
                        value: controller.position_f64(),
                        max: track.read().1.duration.unwrap_or_default() as f64,
                    }

                    div { class: "timing-progress",
                        span { "{controller.position_display()}" }
                        span { "{track_duration}" }
                    }
                }

                div { class: "player-controls",
                    button {
                        class: if !is_shuffle { "control disabled" } else { "control" },
                        onclick: move |_| {
                            controller.shuffle().toggle();
                        },
                        ShuffleIcon { size: 24, filled: is_shuffle }
                    }

                    div { class: "queue-controls",
                        button {
                            class: if !has_previous() { "control disabled" } else { "control" },
                            onclick: move |_| {
                                controller.previous();
                            },
                            PreviousIcon { filled: has_previous() }
                        }

                        button {
                            class: "control",
                            onclick: move |_| {
                                controller.toggle_play();
                            },
                            PlayIconCircle { size: 48, is_playing }
                        }

                        button {
                            class: if !has_next() { "control disabled" } else { "control" },
                            onclick: move |_| {
                                controller.next();
                            },
                            NextIcon { size: 24, filled: has_next() }
                        }
                    }

                    button {
                        class: if !is_random { "control disabled" } else { "control" },
                        onclick: move |_| {
                            controller.random().toggle();
                        },
                        RandomIcon { filled: is_random }
                    }
                }
            }
        }
    }
}
