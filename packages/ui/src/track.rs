use cadence_player::PlayerCommand;
use dioxus::prelude::*;
use tokio::sync::mpsc::Sender;

use crate::{IsPlaying, Queue, album_card::Song};

#[component]
pub fn Track(track: Song) -> Element {
    let sender = use_context::<Sender<PlayerCommand>>();

    rsx!(
        div {
            class: "track-item",
            onclick: move |_| {
                let sender = sender.clone();
                let track_id = track.id.clone();

                spawn(async move {
                    sender.send(PlayerCommand::QueueNow(track_id)).await.unwrap();
                });
                consume_context::<Queue>().append_and_set_current(track.clone());
                consume_context::<IsPlaying>().toggle();
            },
            "{track.title}"
        }
    )
}
