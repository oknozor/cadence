use crate::search::{SearchInput, SearchResults, SearchResultsEmpty};
use crate::shared::VerticalScroller;
use cadence_core::hooks::use_search_results;
use dioxus::prelude::*;

#[component]
pub fn SearchView() -> Element {
    let mut input = use_signal(String::new);
    let mut search_results = use_search_results();

    let oninput = move |value: String| {
        input.set(value.clone());
        search_results.call(value)
    };

    let content = match search_results.value() {
        Some(Ok(search_results)) if search_results.read().is_empty() && input().is_empty() => {
            rsx! {
                SearchResultsEmpty {}
            }
        }
        None if input().is_empty() => rsx! {
            SearchResultsEmpty {}
        },
        Some(Ok(search_results)) => rsx! {
            VerticalScroller {
                SearchResults { search_results }
            }
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
        {content}
    }
}
