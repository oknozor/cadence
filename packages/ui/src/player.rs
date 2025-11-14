use crate::{CurrentTrack, IsPlaying};
use cadence_player::PlayerCommand;
use dioxus::prelude::*;
use tokio::sync::mpsc::Sender;

#[component]
pub fn Player() -> Element {
    let mut is_playing: IsPlaying = use_context();
    let sender: Sender<PlayerCommand> = use_context();
    let current_track: CurrentTrack = use_context();

    rsx! {
        div {
            class: "player-container",

            if let Some(track) = current_track.track.read().as_ref() {
                div {
                    class: "track-info",
                    h3 { "{track.title}" }
                    p { "{track.artist} - {track.album}" }
                }

                div {
                    class: "player-controls",
                    button {
                        onclick: move |_| {},
                        "⏮ Previous"
                    }

                    button {
                        onclick: move |_| {
                                let sender = sender.clone();
                                if is_playing.is_playing() {
                                    let on_play_sender = sender.clone();
                                    spawn(async move { on_play_sender.send(PlayerCommand::Pause).await.unwrap() });
                                    is_playing.toggle();
                                } else {
                                    let on_pause_sender = sender.clone();
                                    spawn(async move { on_pause_sender.send(PlayerCommand::Play).await.unwrap() });
                                    is_playing.toggle();
                                }
                            },
                        if is_playing.is_playing() {
                            "⏸ Pause"
                        } else {
                            "▶ Play"
                        }
                    }

                    button {
                        onclick: move |_| {},
                        "Next ⏭"
                    }
                }
            } else {
                div {
                    class: "no-track",
                    p { "No track loaded" }
                }
            }
        }
    }
}
