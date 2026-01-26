use cadence_core::{
    model::RadioStation,
    state::{ControllerStoreExt, CONTROLLER},
};
use dioxus::prelude::*;

use crate::components::{AnimatedBars, ItemRow, RadioIcon};

#[component]
pub fn RadioStationItem(station: RadioStation, on_click: EventHandler<RadioStation>) -> Element {
    let controller = CONTROLLER.resolve();
    let name = station.name.clone();
    let is_active = use_memo(move || {
        controller.current_radio()()
            .map(|radio| radio.name == name)
            .unwrap_or_default()
    });
    let is_paused = use_memo(move || !controller.is_playing()());

    let thumbnail = rsx! {
        div { class: "radio-thumbnail",
            RadioIcon { size: 50 }
        }
    };

    let content = rsx! {
        RadioItemInfo { radio: station.clone(), is_active, is_paused }
    };

    let callback = move |_| on_click.call(station.clone());

    rsx! {
        ItemRow { thumbnail, content, action: None, callback }
    }
}

/// Custom item info for radio stations with rolling URL animation when active
#[component]
fn RadioItemInfo(
    radio: RadioStation,
    is_active: ReadSignal<bool>,
    is_paused: ReadSignal<bool>,
) -> Element {
    let active = is_active();

    rsx! {
        div { class: "item-info",
            div { class: "item-line",
                if active {
                    AnimatedBars { size: 12, paused: is_paused }
                }
                span { class: "item-primary", "{radio.name}" }
            }
            div { class: if active { "radio-url-container rolling" } else { "radio-url-container" },
                span { class: "item-secondary radio-url", "{radio.stream_url}" }
            }
        }
    }
}
