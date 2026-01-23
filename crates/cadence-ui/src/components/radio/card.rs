use cadence_core::model::RadioStation;
use dioxus::prelude::*;

use crate::components::{ItemInfo, ItemRow, RadioIcon};

#[component]
pub fn RadioStationItem(station: RadioStation, on_click: EventHandler<String>) -> Element {
    let station_id = station.id.clone();

    let thumbnail = rsx! {
        div { class: "radio-thumbnail",
            RadioIcon { size: 50 }
        }
    };

    let content = rsx! {
        ItemInfo {
            primary: station.name,
            secondary: station.stream_url,
            is_active: false,
            is_paused: false,
        }
    };

    let callback = move |_| on_click.call(station_id.clone());

    rsx! {
        ItemRow {
            thumbnail,
            content,
            action: None,
            callback,
        }
    }
}
