use dioxus::prelude::*;
use dioxus_primitives::progress::{Progress, ProgressIndicator};

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
