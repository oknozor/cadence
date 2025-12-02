use crate::progress::PlayerProgress;
use crate::queue::Queue;
use crate::thumbnails::Thumbnail;
use crate::{icons::play::PlayIcon, items::ItemInfo};
use cadence_core::state::{CONTROLLER, ControllerExt, ControllerStoreExt};
use dioxus::prelude::*;

#[component]
pub fn Player(expand: WriteSignal<bool>) -> Element {
    let mut controller = CONTROLLER.resolve();

    let content = if let Some(track) = controller.current() {
        let primary = track.read().1.title.clone();
        let secondary = track.read().1.artist.clone();

        rsx! {
            div {
                class: "player",
                onclick: move |_| {
                    let value = !*expand.read();
                    expand.set(value);
                    debug!("Player clicked");
                },
                div { class: "track-container",
                    if let Some(cover) = track.read().1.cover_art.as_ref() {
                        Thumbnail { src: cover, name: track().1.title, size: 32 }
                    }
                    ItemInfo { primary, secondary }
                }
                div { class: "player-controls",
                    button {
                        onclick: move |_| {
                            controller.toggle_play();
                        },
                        PlayIcon { size: 24, is_playing: controller.is_playing() }
                    }
                }
            }
            PlayerProgress {
                value: controller.position_f64(),
                max: track.read().1.duration.unwrap_or_default() as f64,
            }
            Queue { expand }
        }
    } else {
        rsx! {
            div {
            }
        }
    };

    rsx! {
        div { class: if expand() { "player-container expanded" } else { "player-container" }, {content} }
    }
}
