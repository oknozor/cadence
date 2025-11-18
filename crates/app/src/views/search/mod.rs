use crate::{components::search::SearchResult, services::subsonic_client::SUBSONIC_CLIENT};
use dioxus::prelude::*;

#[component]
pub fn SearchView() -> Element {
    let search_results = use_signal(|| vec![]);

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div {
            class: "search-view",
            SearchInput { search_results }
            SearchResults { search_results }
        }
    }
}

#[component]
pub fn SearchInput(search_results: WriteSignal<Vec<SearchResult>>) -> Element {
    let client = SUBSONIC_CLIENT.cloned().unwrap();

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        input {
            id: "search-input",
            type: "search",
            placeholder: "Search...",
            oninput: move |event| {
                let client = client.clone();
                spawn(async move {
                    *search_results.write() = client.search(&event.value()).await.unwrap();
                });
            }
        }
    }
}

#[component]
pub fn SearchResults(search_results: ReadSignal<Vec<SearchResult>>) -> Element {
    rsx! {
        div {
            display: "flex",
            flex_direction: "column",
            for result in search_results.read().iter() {
                if let SearchResult::Artist {name, .. } = result {
                    p {
                        "{name}"
                    }
                }
                else if let SearchResult::Album {name, .. } = result {
                    p {
                        "{name}"
                    }
                }
                else if let SearchResult::Song {name, .. } = result {
                    p {
                        "{name}"
                    }
                }
            }
        }
    }
}
