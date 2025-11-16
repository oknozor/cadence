use crate::components::album_card::Song;
use crate::components::icons::dots::DotIcon;
use crate::context::{IsPlaying, Queue};
use cadence_player::PlayerCommand;
use dioxus::prelude::*;
use tokio::sync::mpsc::Sender;

#[component]
pub fn Track(track: Song) -> Element {
    let sender = use_context::<Sender<PlayerCommand>>();
    rsx!(
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div {
            class: "track-row",
            onclick: move |_| {
                let sender = sender.clone();
                let track_id = track.id.clone();

                spawn(async move {
                    sender.send(PlayerCommand::QueueNow(track_id)).await.unwrap();
                });
                consume_context::<Queue>().append_and_set_current(track.clone());
                consume_context::<IsPlaying>().toggle();
            },
            p {
                class: "track-title",
                "{track.title}"
            },
            DotIcon  { size: 18 }
        }
    )
}
