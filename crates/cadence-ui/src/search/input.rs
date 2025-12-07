use dioxus::prelude::*;

use crate::shared::SearchIcon;

#[component]
pub fn SearchInput(oninput: EventHandler<String>) -> Element {
    rsx! {
        div { class: "search-input-container",
            SearchIcon { size: 18, filled: false }
            input {
                id: "search-input",
                r#type: "search",
                placeholder: "Search...",
                oninput: move |e| oninput.call(e.value()),
            }
        }
    }
}
