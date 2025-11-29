use crate::icons::dots::DotIcon;
use crate::items::ItemInfo;
use cadence_core::model::Song;
use cadence_core::state::{CONTROLLER, ControllerExt, ControllerStoreExt};
use dioxus::prelude::*;

#[component]
pub fn TrackRow(song: Song) -> Element {
    let mut controller = CONTROLLER.resolve();
    let active = controller.is_active(&song.id);
    let paused = !*controller.is_playing().read();

    rsx!(
        div {
            class: "track-row",
            onclick: move |_| {
                debug!("TrackRow clicked {:?}", song);
                controller.queue_now(song.clone());
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
