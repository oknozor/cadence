use dioxus::prelude::*;
use dioxus_primitives::progress::{self, ProgressIndicatorProps, ProgressProps};

#[component]
pub fn Progress(props: ProgressProps) -> Element {
    let current = use_memo(move || {
        if let Some(value) = *props.value.read() {
            let minutes = value / 60.0;
            let seconds = value % 60.0;
            format!("{}:{:02}", minutes as u32, seconds as u32)
        } else {
            "00:00".to_string()
        }
    });

    let max_in_min_sec = use_memo(move || {
        let minutes = *props.max.read() / 60.0;
        let seconds = *props.max.read() % 60.0;
        format!("{}:{}", minutes as u32, seconds as u32)
    });

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div {
            class: "progress-container",
            progress::Progress {
                class: "progress",
                value: props.value,
                max: props.max,
                attributes: props.attributes,
                {props.children}
            }
            div {
                class: "progress-info",
                span { "{current}" }
                span { "{max_in_min_sec}" }
            }
        }
    }
}

#[component]
pub fn ProgressIndicator(props: ProgressIndicatorProps) -> Element {
    rsx! {
        progress::ProgressIndicator { class: "progress-indicator", attributes: props.attributes, {props.children} }
    }
}
