use crate::icons::play::PlayIcon;
use crate::items::ItemInfo;
use crate::progress::Progress;
use crate::progress::ProgressIndicator;
use crate::thumbnails::Thumbnail;
use cadence_core::PlayerCommand;
use cadence_core::hooks::use_command_sender;
use cadence_core::hooks::use_player_state;
use cadence_core::hooks::{use_current_track, use_playback_position};
use dioxus::prelude::*;

#[component]
pub fn Player() -> Element {
    let mut state = use_player_state();
    let playback_position = use_playback_position();
    let current_track = use_current_track();
    let command_sender = use_command_sender();

    rsx! {
        div { class: "player-container",
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
                            Thumbnail { src: cover, name: &track.title, size: 32 }
                        }
                        ItemInfo {
                            primary: track.title,
                            secondary: track.artist,
                            active: false,
                            paused: false,
                        }
                    
                    }
                    div { class: "player-controls",
                        button {
                            onclick: move |_| {
                                if *state.is_playing().read() {
                                    let sender = command_sender.clone();
                                    spawn(async move {
                                        sender.clone().send(PlayerCommand::Pause).await.unwrap()
                                    });
                                    state.toggle();
                                } else {
                                    let sender = command_sender.clone();
                                    spawn(async move { sender.send(PlayerCommand::Play).await.unwrap() });
                                    state.toggle();
                                }
                            },
                            PlayIcon { size: 24, is_playing: state.is_playing() }
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
