use crate::shared::{DotIcon, ItemInfo};
use cadence_core::model::Song;
use cadence_core::state::{CONTROLLER, ControllerExt, ControllerStoreExt};
use dioxus::prelude::*;

#[component]
pub fn TrackRow(song: Song) -> Element {
    let mut controller = CONTROLLER.resolve();
    let id = song.id.clone();
    let is_active = use_memo(move || {
        let current = controller.current_song_id()();
        current == Some(id.clone())
    });

    let is_paused = !*controller.is_playing().read();

    rsx!(
        div {
            class: "track-row",
            onclick: move |_| {
                controller.play_now(song.clone());
            },
            ItemInfo {
                primary: song.title.clone(),
                secondary: song.artist.clone(),
                is_active,
                is_paused,
            }
            DotIcon { size: 28 }
        }
    )
}
