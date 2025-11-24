use dioxus::prelude::*;

#[component]
pub fn SearchInput(oninput: EventHandler<String>) -> Element {
    rsx! {
        input {
            id: "search-input",
            r#type: "search",
            placeholder: "Search...",
            oninput: move |e| oninput.call(e.value()),
        }
    }
}
