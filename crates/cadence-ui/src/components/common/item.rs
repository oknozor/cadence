use crate::components::AnimatedBars;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ItemInfoProps {
    pub primary: ReadSignal<String>,

    pub secondary: ReadSignal<String>,

    #[props(default = ReadSignal::new(Signal::new(false)))]
    pub is_active: ReadSignal<bool>,

    #[props(default = ReadSignal::new(Signal::new(false)))]
    pub is_paused: ReadSignal<bool>,
}

#[component]
pub fn ItemInfo(props: ItemInfoProps) -> Element {
    let is_active = *props.is_active.read();

    rsx! {
        div { class: "item-info",
            div { class: "item-line",
                if is_active {
                    AnimatedBars { size: 12, paused: props.is_paused }
                }
                span { class: "item-primary", "{props.primary}" }
            }
            span { class: "item-secondary", "{props.secondary}" }
        }
    }
}

#[component]
pub fn ItemRow(
    thumbnail: Option<Element>,
    content: Element,
    action: Option<Element>,
    callback: EventHandler,
) -> Element {
    rsx! {
        div { class: "item-row",
            div {
                class: "item-row-start",
                onclick: move |_| {
                    callback.call(());
                },
                if let Some(thumbnail) = thumbnail {
                    {thumbnail}
                }
                {content}
            }
            if let Some(action) = action {
                div { class: "item-row-action", {action} }
            }
        }
    }
}
