use crate::components::{DotIcon, ItemInfo};
use cadence_core::model::Song;
use cadence_core::state::{CONTROLLER, ControllerExt, ControllerStoreExt};
use dioxus::prelude::*;

#[component]
pub fn TrackRow(song: Song, action_clicked: EventHandler<Song>) -> Element {
    let mut controller = CONTROLLER.resolve();
    let id = song.id.clone();
    let is_active = use_memo(move || {
        let current = controller.current_song_id()();
        current == Some(id.clone())
    });

    let is_paused = !*controller.is_playing().read();

    // FIXME: we need a song store
    let song_clone = song.clone();

    rsx!(
        div {
            class: "track-row",
            onclick: move |_| {
                controller.play_now(song_clone.clone());
            },
            ItemInfo {
                primary: song.title.clone(),
                secondary: song.artist.clone(),
                is_active,
                is_paused,
            }
            button {
                onclick: move |event| {
                    action_clicked.call(song.clone());
                    event.stop_propagation();
                },
                DotIcon { size: 28 }
            }
        }
    )
}
