use cadence_core::model::RadioStation;
use dioxus::prelude::*;

use crate::components::RadioStationItem;

#[component]
pub fn RadioStationList(
    title: String,
    stations: Vec<RadioStation>,
    on_click: EventHandler<String>,
) -> Element {
    rsx! {
        div { class: "radio-station-list",
            h2 { "{title}" }

            div { class: "radio-station-items",
                for station in stations {
                    RadioStationItem { station, on_click }
                }
            }
        }
    }
}

