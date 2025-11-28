use crate::icons::dots::DotIcon;
use crate::items::ItemInfo;
use cadence_core::PlayerCommand;
use cadence_core::hooks::{use_command_sender, use_player_state, use_queue_state};
use cadence_core::model::Song;
use dioxus::prelude::*;

#[component]
pub fn TrackRow(song: Song) -> Element {
    let sender = use_command_sender();
    let mut player = use_player_state();
    let mut queue = use_queue_state();
    let active = player.song().as_ref() == Some(&song.id);
    let paused = !*player.is_playing().read();

    rsx!(
        div {
            class: "track-row",
            onclick: move |_| {
                let sender = sender.clone();

                let track = song.clone();
                spawn(async move {
                    sender
                        .send(PlayerCommand::QueueNow(track))
                        .await
                        .unwrap();
                });
                queue.append_and_set_current(song.clone());
                player.set_playing(song.id.clone());
            },
            ItemInfo {
                primary: song.title.clone(),
                secondary: song.artist.clone(),
                active,
                paused,
            }
            DotIcon { size: 28 }
        }
    )
}
