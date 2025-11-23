use crate::icons::dots::DotIcon;
use crate::items::ItemInfo;
use cadence_core::PlayerCommand;
use cadence_core::hooks::{use_command_sender, use_player_state, use_queue_state};
use cadence_core::model::Song;
use dioxus::prelude::*;

#[component]
pub fn TrackRow(track: Song) -> Element {
    let sender = use_command_sender();
    let mut player = use_player_state();
    let mut queue = use_queue_state();
    let active = player.song().as_ref() == Some(&track.id);
    let paused = !*player.is_playing().read();

    rsx!(
        div {
            class: "track-row",
            onclick: move |_| {
                let sender = sender.clone();
                let track_id = track.id.clone();

                spawn(async move {
                    sender.send(PlayerCommand::QueueNow(track_id)).await.unwrap();
                });
                queue.append_and_set_current(track.clone());
                player.set_playing(track.id.clone());
            },
            ItemInfo { primary: track.title.clone(), secondary: track.artist.clone(), active, paused }
            DotIcon  { size: 18 }
        }
    )
}
