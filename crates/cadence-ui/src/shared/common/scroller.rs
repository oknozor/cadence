use dioxus::prelude::*;

#[component]
pub fn HorizontalScroller(children: Element) -> Element {
    rsx! {
        div { class: "horizontal-scroller", {children} }
    }
}

#[component]
pub fn VerticalScroller(children: Element) -> Element {
    rsx! {
        div { class: "vertical-scroller", {children} }
    }
}
