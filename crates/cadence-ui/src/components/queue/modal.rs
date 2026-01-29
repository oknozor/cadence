use cadence_core::state::{CONTROLLER, ControllerStoreExt};
use dioxus::prelude::*;

use crate::components::{MenuModal, SongItemInfo};

#[component]
pub fn QueueModal(open: Signal<bool>) -> Element {
    let controller = CONTROLLER.resolve();

    // Get queue songs sorted by index
    let queue_songs = use_memo(move || {
        let binding = controller.queue_store();
        let queue = binding.read();
        let mut songs: Vec<_> = queue
            .iter()
            .map(|(idx, store)| {
                let (_, song) = store.read().clone();
                (idx, song)
            })
            .collect();
        songs.sort_by_key(|(idx, _)| *idx);
        songs
    });

    let current_idx = use_memo(move || *controller.current_idx().read());

    rsx! {
        MenuModal { open,
            div { class: "queue-modal-header",
                h2 { "Queue" }
            }

            div { class: "queue-modal-content",
                if queue_songs().is_empty() {
                    div { class: "queue-empty",
                        p { "Your queue is empty" }
                    }
                } else {
                    div { class: "queue-section",
                        h3 { "Now Playing" }
                        for (idx , song) in queue_songs().iter().filter(|(idx, _)| *idx == current_idx()).collect::<Vec<_>>() {
                            SongItemInfo {
                                key: "{idx}",
                                song: song.clone(),
                                thumbnail_size: 48,
                            }
                        }
                    }

                    if queue_songs().iter().any(|(idx, _)| *idx > current_idx()) {
                        div { class: "queue-section",
                            h3 { "Next Up" }
                            for (idx , song) in queue_songs().iter().filter(|(idx, _)| *idx > current_idx()).collect::<Vec<_>>() {
                                SongItemInfo {
                                    key: "{idx}",
                                    song: song.clone(),
                                    thumbnail_size: 48,
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
