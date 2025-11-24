use cadence_core::hooks::use_search_results;
use cadence_ui::search::{SearchInput, SearchResults};
use dioxus::prelude::*;

#[component]
pub fn SearchView() -> Element {
    let mut input = use_signal(|| String::new());
    let mut search_results = use_search_results();

    let empty = rsx! {
        div { class: "search-empty",
            span { class: "text-primary", "What do you want to listen to?" }
            span { class: "text-secondary", "Search for artists, albums, songs, or playlists" }
        }
    };

    let oninput = move |value: String| {
        input.set(value.clone());
        search_results.call(value)
    };

    let content = match search_results.value() {
        Some(Ok(search_results)) if search_results.read().is_empty() && input().is_empty() => empty,
        None if input().is_empty() => empty,
        Some(Ok(search_results)) => rsx! {
            SearchResults { search_results }
        },
        Some(Err(err)) => rsx! {
            div { class: "search-results", "Error: {err}" }
        },
        None => rsx! {
            div {}
        },
    };

    rsx! {
        SearchInput { oninput }
        div { class: "view", {content} }
    }
}
