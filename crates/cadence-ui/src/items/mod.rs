use crate::icons::animated_bars::AnimatedBars;
use dioxus::prelude::*;

#[component]
pub fn ItemInfo(primary: String, secondary: String, active: bool, paused: bool) -> Element {
    rsx! {
        div { class: "item-info",
            div { class: "item-line",
                if active {
                    AnimatedBars { size: 12, paused }
                }
                span { class: "item-primary", "{primary}" }
            }
            span { class: "item-secondary", "{secondary}" }
        }
    }
}
