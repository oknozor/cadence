use cadence_core::state::{CONTROLLER, ControllerStoreExt};
use dioxus::prelude::*;

use crate::items::ItemInfo;

#[component]
pub fn Queue() -> Element {
    let controller = CONTROLLER.resolve();
    let queue = controller.queue_store();
    let is_paused = !*controller.is_playing().read();
    let mut expanded = use_signal(|| true);

    rsx! {
        div { class: if expanded() { "music-queue expanded" } else { "music-queue" },
            div { class: "queue-header",
                div { class: "drag-handle",
                    ondrag: move |_| expanded.set(!expanded()) }
                div { class: "queue-title", "Queue" }
            }
            div { class: "queue-list",
                for (_ , song) in queue.read().iter() {
                    ItemInfo {
                        primary: song.read().1.title.clone(),
                        secondary: song.read().1.artist.clone(),
                        is_active: song.read().0,
                        is_paused,
                    }
                }
            }
        }
    }
}
