use dioxus::prelude::*;

use crate::shared::{Progress, ProgressIndicator};

#[component]
pub fn PlayerProgress(value: ReadSignal<Option<f64>>, max: f64) -> Element {
    rsx! {
        div { class: "progress-container",
            Progress { class: "progress", value, max,
                ProgressIndicator { class: "progress-indicator" }
            }
        }
    }
}
