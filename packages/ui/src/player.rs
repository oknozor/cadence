use crate::components::progress::Progress;
use crate::{IsPlaying, Queue};
use cadence_player::PlayerCommand;
use dioxus::prelude::*;
use tokio::sync::mpsc::Sender;

#[component]
pub fn Player() -> Element {
    let mut is_playing: IsPlaying = use_context();

    rsx! {
        div {
            class: "player-container",

            if let Some(track) = consume_context::<Queue>().get_current() {
                div {
                    class: "track-info",
                    h3 { "{track.title}" }
                    p { "{track.artist} - {track.album}" }
                }

                div {
                    class: "player-controls",
                    button {
                        onclick: move |_| {
                            let onprev_sender:Sender<PlayerCommand> = consume_context();
                            let mut queue: Queue = consume_context();
                            let song = queue.previous();

                            if let Some(song) = song {
                                spawn(async move {
                                    onprev_sender.send(PlayerCommand::QueueNow(song.id.clone())).await.unwrap()
                                });
                            }

                        },
                        "⏮ Previous"
                    }

                    button {
                        onclick: move |_| {
                                let sender: Sender<PlayerCommand> = consume_context();
                                if is_playing.is_playing() {
                                    spawn(async move { sender.send(PlayerCommand::Pause).await.unwrap() });
                                    is_playing.toggle();
                                } else {
                                    let sender = sender.clone();
                                    spawn(async move { sender.send(PlayerCommand::Play).await.unwrap() });
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
                        onclick: move |_| {
                            let onnext_sender:Sender<PlayerCommand> = consume_context();
                            let mut queue: Queue = consume_context();
                            let song = queue.skip();

                            if let Some(song) = song {
                                spawn(async move {
                                    onnext_sender.send(PlayerCommand::QueueNow(song.id.clone())).await.unwrap()
                                });
                            }
                        },
                        "Next ⏭"
                    }

                    Progress {
                        value: 30.0,
                        max: 100.0,
                        div {}
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
