use crate::album::AlbumCover;
use crate::icons::list::ListIcon;
use crate::icons::next::NextIcon;
use crate::icons::play::PlayIcon;
use crate::icons::previous::PreviousIcon;
use crate::icons::random::RandomIcon;
use crate::icons::shuffle::ShuffleIcon;
use crate::progress::PlayerProgress;
use crate::queue::Queue;
use crate::thumbnails::Thumbnail;
use crate::views::Route;
use crate::{icons::play::PlayIconCircle, items::ItemInfo};
use cadence_core::state::{ControllerExt, ControllerStoreExt, CONTROLLER};
use dioxus::html::a::width;
use dioxus::prelude::*;

#[component]
pub fn Player() -> Element {
    let mut controller = CONTROLLER.resolve();
    let nav = navigator();

    let content = if let Some(track) = controller.current() {
        let primary = track.read().1.title.clone();
        let secondary = track.read().1.artist.clone();

        rsx! {
            div { class: "player", onclick: move |_| {},
                div { class: "track-container",
                    if let Some(cover) = track.read().1.cover_art.as_ref() {
                        Thumbnail { src: cover, name: track().1.title, size: 32 }
                    }
                    ItemInfo { primary, secondary }
                }
                div { class: "player-controls",
                    button {
                        onclick: move |_| {
                            nav.push(Route::NowPlayingView {});
                        },
                        ListIcon {}
                    }
                    button {
                        onclick: move |_| {
                            controller.toggle_play();
                        },
                        PlayIconCircle { is_playing: controller.is_playing() }
                    }
                }
            }
            PlayerProgress {
                value: controller.position_f64(),
                max: track.read().1.duration.unwrap_or_default() as f64,
            }
        }
    } else {
        rsx! {
            div {
            }
        }
    };

    rsx! {
        div { class: "player-container", {content} }
    }
}

#[component]
pub fn FullScreenPlayer() -> Element {
    let controller = CONTROLLER.resolve();
    let queue = controller.queue_store();
    let current = controller.current();
    let expand = use_signal(|| false);
    let is_playing = controller.is_playing()();
    let is_shuffle = controller.shuffle()();
    let is_random = controller.random()();
    let has_next = use_memo(move || {
        let current = *controller.current_idx().read();
        let last = controller.queue_store().read().len();
        current != last
    });
    let has_previous = use_memo(move || {
        let current = *controller.current_idx().read();
        current == 0
    });

    rsx! {
        div { class: "player-fullscreen",
            if let Some(track) = current {
                if let Some(src) = track.read().1.cover_art.clone() {
                    AlbumCover { src, width: "200px" }
                }
                ItemInfo {
                    primary: track.read().1.title.clone(),
                    secondary: track.read().1.artist.clone(),
                }
            }
            div { class: "player-controls",
                button { class: if !is_shuffle { "control disabled" } else { "control" },
                    ShuffleIcon { size: 24, filled: is_shuffle }
                }

                div { class: "queue-controls",
                    button { class: if !has_previous() { "control disabled" } else { "control" },
                        PreviousIcon { filled: has_previous() }
                    }

                    button { class: "control",
                        PlayIconCircle { size: 48, is_playing: controller.is_playing() }
                    }

                    button { class: if !has_next() { "control disabled" } else { "control" },
                        NextIcon { size: 24, filled: has_next() }
                    }
                }

                button { class: if !is_random { "control disabled" } else { "control" },
                    RandomIcon { filled: is_random }
                }
            }
            Queue { expand }
        }
    }
}
