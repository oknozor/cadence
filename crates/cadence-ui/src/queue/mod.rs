use cadence_core::state::{CONTROLLER, ControllerExt, ControllerStoreExt};
use dioxus::prelude::*;

use crate::{album::AlbumCover, items::ItemInfo};

#[component]
pub fn Queue() -> Element {
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
            div { class: "now-playing-view",
                div { class: "now-playing",
                if let Some(src) = cover_art.read().clone() {
                    AlbumCover { src  }
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
}
