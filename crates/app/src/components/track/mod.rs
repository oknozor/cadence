use crate::components::album_card::Song;
use crate::components::icons::animated_bars::AnimatedBars;
use crate::components::icons::dots::DotIcon;
use crate::context::{IsPlaying, Queue};
use cadence_player::PlayerCommand;
use dioxus::prelude::*;
use tokio::sync::mpsc::Sender;

#[component]
pub fn Track(track: Song) -> Element {
    let sender = use_context::<Sender<PlayerCommand>>();
    let is_playing = consume_context::<IsPlaying>().song().as_ref() == Some(&track.id);

    rsx!(
        div {
            class: "track-row",
            onclick: move |_| {
                let sender = sender.clone();
                let track_id = track.id.clone();

                spawn(async move {
                    sender.send(PlayerCommand::QueueNow(track_id)).await.unwrap();
                });
                consume_context::<Queue>().append_and_set_current(track.clone());
                consume_context::<IsPlaying>().set_playing(track.id.clone());
            },
            if is_playing {
                AnimatedBars { size: 18 }
            }
            span {
                class: "track-title",
                "{track.title}"
            },
            DotIcon  { size: 18 }
        }
    )
}
