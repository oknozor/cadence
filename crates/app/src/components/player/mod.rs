use crate::components::icons::play::PlayIcon;
use crate::components::progress::Progress;
use crate::components::progress::ProgressIndicator;
use crate::context::{IsPlaying, Queue};
use cadence_player::PlayerCommand;
use dioxus::prelude::*;
use tokio::sync::broadcast;
use tokio::sync::mpsc::Sender;

#[component]
pub fn Player() -> Element {
    let mut is_playing: IsPlaying = use_context();
    let position_tx: broadcast::Sender<u64> = use_context();
    let mut position = use_signal(|| None::<f64>);

    use_effect(move || {
        let mut tx = position_tx.subscribe();
        spawn(async move {
            while let Ok(new_progress) = tx.recv().await {
                position.set(Some(new_progress as f64));
            }
        });
    });

    rsx! {
        div {
            class: "player-container",
            if let Some(track) = consume_context::<Queue>().get_current() {
                div {
                    display: "flex",
                    flex_direction: "row",
                    align_items: "center",
                    padding_left: "10px",
                    div {
                        class: "track-container",
                        flex: "column",
                        flex_grow: 1,
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
                            PlayIcon {
                                size: 24,
                                is_playing: is_playing.to_signal()
                            }
                        }
                    }
                }
                Progress {
                    value: position,
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
