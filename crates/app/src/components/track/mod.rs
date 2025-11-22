use crate::components::album_card::Song;
use crate::context::{IsPlaying, Queue};
use cadence_player::PlayerCommand;
use cadence_ui::icons::animated_bars::AnimatedBars;
use cadence_ui::icons::dots::DotIcon;
use cadence_ui::items::ItemInfo;
use dioxus::prelude::*;
use tokio::sync::mpsc::Sender;

#[component]
pub fn Track(track: Song) -> Element {
    let sender = use_context::<Sender<PlayerCommand>>();
    let playing = consume_context::<IsPlaying>();
    let active = playing.song().as_ref() == Some(&track.id);
    let paused = !*playing.is_playing().read();

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
            ItemInfo { primary: track.title.clone(), secondary: track.artist.clone(), active, paused }
            DotIcon  { size: 18 }
        }
    )
}
