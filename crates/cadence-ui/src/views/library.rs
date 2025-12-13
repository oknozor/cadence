use dioxus::prelude::*;

#[component]
pub fn LibraryView() -> Element {
    rsx! {
        div { class: "view", "toto" }
    }
}
