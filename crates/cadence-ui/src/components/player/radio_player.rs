use crate::components::{PlayIconCircle, RadioIcon};
use cadence_core::model::RadioStation;
use cadence_core::state::{ControllerExt, ControllerStoreExt, CONTROLLER};
use dioxus::prelude::*;

/// A mini player for radio streams, similar to the track Player but without
/// ListIcon and PlayerProgress since radio streams don't have a duration.
#[component]
pub fn RadioPlayer() -> Element {
    let mut controller = CONTROLLER.resolve();

    let content = if let Some(radio) = controller.current_radio()() {
        rsx! {
            div { class: "player radio-player",
                div { class: "track-container",
                    div { class: "radio-player-thumbnail",
                        RadioIcon { size: 32 }
                    }
                    RadioPlayerInfo { radio }
                }
                div { class: "player-controls",
                    button {
                        onclick: move |_| {
                            controller.toggle_play();
                        },
                        PlayIconCircle { is_playing: controller.is_playing() }
                    }
                }
            }
        }
    } else {
        rsx! {
            div {}
        }
    };

    rsx! {
        div { class: "player-container", {content} }
    }
}

/// Displays the radio stream info with a rolling URL animation
#[component]
fn RadioPlayerInfo(radio: RadioStation) -> Element {
    rsx! {
        div { class: "item-info",
            span { class: "item-primary", "{radio.name}" }
            div { class: "radio-url-container rolling",
                span { class: "item-secondary radio-url", "{radio.stream_url}" }
            }
        }
    }
}

