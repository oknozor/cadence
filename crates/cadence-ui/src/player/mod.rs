use crate::{
    progress::PlayerProgress,
    shared::{ItemInfo, ListIcon, PlayIconCircle, Thumbnail},
    views::Route,
};
use cadence_core::state::{ControllerExt, ControllerStoreExt, CONTROLLER};
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
                max: track.read().1.duration.unwrap_or_default() as f64
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
