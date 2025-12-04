use cadence_core::state::{CONTROLLER, ControllerExt, ControllerStoreExt};
use dioxus::prelude::*;

use crate::items::ItemInfo;

#[component]
pub fn Queue(expand: ReadSignal<bool>) -> Element {
    let controller = CONTROLLER.resolve();
    let queue = controller.queue_store();
    let current = controller.current();
    let cover_art = use_memo(move || {
        if let Some(current) = current {
            current.read().1.cover_art.clone()
        } else {
            None
        }
    });

    let is_paused = !*controller.is_playing().read();

    rsx! {
        div { class: if expand() { "queue-container expanded" } else { "queue-container" },
            div {
                div {
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
}
