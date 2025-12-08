use std::time::Duration;

use cadence_core::state::{CONTROLLER, ControllerExt};
use dioxus::prelude::*;

use crate::shared::{Progress, ProgressIndicator};

#[component]
pub fn PlayerProgress(value: ReadSignal<Option<f64>>, max: f64) -> Element {
    let mut controller = CONTROLLER.resolve();
    let mut witdh = use_signal(|| 0.0);

    rsx! {
        // Fixme: the seek logic should be in progress and track the size of the component.
        div {
            class: "progress-container",
            onmounted: move |evt: Event<MountedData>| async move {
                if let Ok(r) = evt.data().get_client_rect().await {
                    witdh.set(r.width());
                }
            },
            onclick: move |event: Event<MouseData>| {
                let coordinates = event.data().element_coordinates();
                let x_pos = coordinates.x;
                let progress = x_pos / witdh();
                let seek_time = progress * max;
                controller.seek(Duration::from_secs(seek_time as u64));
            },
            Progress {
                id: "player-progress",
                class: "progress",
                value,
                max,
                ProgressIndicator { class: "progress-indicator" }
            }
        }
    }
}
