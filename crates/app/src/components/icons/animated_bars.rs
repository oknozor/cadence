use dioxus::prelude::*;

#[component]
pub fn AnimatedBars(size: ReadSignal<u8>) -> Element {
    rsx! {
        div {
            class: "music-bars",
            width: "{size}",
            div {}
            div {}
            div {}
        }
    }
}
