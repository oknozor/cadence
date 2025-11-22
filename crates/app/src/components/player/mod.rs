use crate::components::progress::Progress;
use crate::components::progress::ProgressIndicator;
use crate::context::{IsPlaying, Queue};
use crate::shared::thumbnails::Thumbnail;
use cadence_player::PlayerCommand;
use cadence_ui::icons::play::PlayIcon;
use dioxus::prelude::*;
use tokio::sync::broadcast;
use tokio::sync::mpsc::Sender;

#[component]
pub fn Player() -> Element {
    let mut playing: IsPlaying = use_context();
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
                        if let Some(cover) = track.cover_art {
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
                                let sender: Sender<PlayerCommand> = consume_context();
                                if *playing.is_playing().read() {
                                    spawn(async move { sender.send(PlayerCommand::Pause).await.unwrap() });
                                    playing.toggle();
                                } else {
                                    let sender = sender.clone();
                                    spawn(async move { sender.send(PlayerCommand::Play).await.unwrap() });
                                    playing.toggle();
                                }
                            },
                            PlayIcon {
                                size: 24,
                                is_playing: playing.is_playing(),
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
