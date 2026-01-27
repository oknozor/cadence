use crate::components::{
    AlbumCover, ItemInfo, LyricsCard, NextIcon, PlayIconCircle, PlayerProgress, PlusIcon,
    PreviousIcon, RandomIcon, ShuffleIcon,
};
use cadence_core::hooks::use_lyrics;
use cadence_core::state::{CONTROLLER, ControllerExt, ControllerStoreExt};
use dioxus::prelude::*;

#[component]
pub fn NowPlayingView() -> Element {
    let mut controller = CONTROLLER.resolve();
    let current = controller.current();
    let is_playing = controller.is_playing()();
    let is_shuffle = controller.shuffle()();
    let is_random = controller.random()();

    // Signals for current track info (for lyrics hook)
    let mut current_artist = use_signal(String::new);
    let mut current_title = use_signal(String::new);
    let mut current_duration = use_signal(|| None::<u64>);

    // Update signals when track changes
    use_effect(move || {
        if let Some(track) = current {
            let track_data = track.read();
            if current_artist() != track_data.1.artist || current_title() != track_data.1.title {
                current_artist.set(track_data.1.artist.clone());
                current_title.set(track_data.1.title.clone());
                current_duration.set(track_data.1.duration.map(|d| d as u64));
            }
        } else {
            if !current_artist().is_empty() || !current_title().is_empty() {
                current_artist.set(String::new());
                current_title.set(String::new());
                current_duration.set(None);
            }
        }
    });

    let lyrics = use_lyrics(current_artist, current_title, current_duration);

    let position_ms = use_memo(move || (controller.position().read().as_millis()) as u64);

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
                    // Toggle between album art and lyrics
                    div { class: "player-visual",
                        if let Some(src) = track.read().1.cover_art.clone() {
                            AlbumCover { src, width: "100%" }
                        }
                    }
                    LyricsCard { lyrics, position_ms }
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
