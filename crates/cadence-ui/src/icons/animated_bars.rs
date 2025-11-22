use dioxus::prelude::*;

#[component]
pub fn AnimatedBars(size: u8, paused: ReadSignal<bool>) -> Element {
    rsx! {
        div {
            class: "music-bars",
            width: "{size}",
            height: "{size}",
            div {
                class: if !paused() { "active" },
            }
            div {
                class: if !paused() { "active" },
            }
            div {
                class: if !paused() { "active" },
            }
        }
    }
}
