use crate::components::progress::Progress;
use crate::components::progress::ProgressIndicator;
use cadence_core::hooks::use_command_sender;
use cadence_core::hooks::use_player_state;
use cadence_core::hooks::{use_current_track, use_playback_position};
use cadence_player::PlayerCommand;
use cadence_ui::icons::play::PlayIcon;
use cadence_ui::thumbnails::Thumbnail;
use dioxus::prelude::*;

#[component]
pub fn Player() -> Element {
    let mut state = use_player_state();
    let playback_position = use_playback_position();
    let current_track = use_current_track();
    let command_sender = use_command_sender();

    rsx! {
        div {
            class: "player-container",
            if let Some(track) = current_track() {
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
                            Thumbnail {
                                src: cover,
                                name: &track.title,
                                size: 32,
                            }
                        }
                        div {
                            class: "track-info",
                            h3 { "{track.title}" },
                            p { "{track.artist}" }
                        }
                    }
                    div {
                        class: "player-controls",
                        button {
                            onclick: move |_| {
                                if *state.is_playing().read() {
                                    let sender = command_sender.clone();
                                    spawn(async move { sender.clone().send(PlayerCommand::Pause).await.unwrap() });
                                    state.toggle();
                                } else {
                                    let sender = command_sender.clone();
                                    spawn(async move { sender.send(PlayerCommand::Play).await.unwrap() });
                                    state.toggle();
                                }
                            },
                            PlayIcon {
                                size: 24,
                                is_playing: state.is_playing(),
                            }
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
