use crate::icons::{
    download::DownloadIcon, play::PlayIcon, plus::PlusIcon, share::ShareIcon, shuffle::ShuffleIcon,
};
use cadence_core::{
    PlayerCommand,
    hooks::{use_command_sender, use_player_state, use_queue_state},
    model::Song,
};
use dioxus::prelude::*;

#[component]
pub fn AlbumActionBar(songs: Vec<Song>) -> Element {
    let mut player = use_player_state();
    let mut queue = use_queue_state();
    let command_sender = use_command_sender();

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
                    onclick: move |_| {
                        let sender = command_sender.clone();
                        if *player.is_playing().read() {
                            spawn(async move {
                                sender.send(PlayerCommand::Pause).await.unwrap();
                            });
                        }
                        else if let Some(first) = songs.get(0).cloned() {
                            let sender = command_sender.clone();
                            let tracks = songs.clone();
                            spawn(async move {
                                sender.send(PlayerCommand::QueueAll(tracks)).await.unwrap();
                            });

                            queue.queue_all(songs.clone());
                            player.set_playing(first.id);
                        }
                    },
                    PlayIcon { size: 48, is_playing: player.is_playing() }
                }
            }
        }
    }
}
