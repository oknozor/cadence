use crate::services::subsonic_client::SUBSONIC_CLIENT;
use dioxus::{CapturedError, prelude::*};

#[component]
pub fn SearchView() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div {
            class: "search-view",
            div {
            }
        }
    }
}
