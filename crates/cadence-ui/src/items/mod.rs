use crate::icons::animated_bars::AnimatedBars;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ItemInfoProps {
    pub primary: ReadSignal<String>,

    pub secondary: ReadSignal<String>,

    #[props(default = ReadSignal::new(Signal::new(false)))]
    pub active: ReadSignal<bool>,

    #[props(default = ReadSignal::new(Signal::new(false)))]
    pub paused: ReadSignal<bool>,
}
#[component]
pub fn ItemInfo(props: ItemInfoProps) -> Element {
    rsx! {
        div { class: "item-info",
            div { class: "item-line",
                if *props.active.read() {
                    AnimatedBars { size: 12, paused: props.paused }
                }
                span { class: "item-primary", "{props.primary}" }
            }
            span { class: "item-secondary", "{props.secondary}" }
        }
    }
}
