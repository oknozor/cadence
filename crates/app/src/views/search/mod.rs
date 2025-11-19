use crate::components::icons::search::SearchIcon;
use crate::shared::thumbnails::{RoundedThumbnail, Thumbnail};
use crate::{components::search::SearchResult, services::subsonic_client::SUBSONIC_CLIENT};
use dioxus::{CapturedError, prelude::*};

#[component]
pub fn SearchView() -> Element {
    let mut input = use_signal(|| String::new());
    let mut search_results = use_action(move |input: String| async move {
        if input.is_empty() {
            return Ok(vec![]);
        }

        let client = SUBSONIC_CLIENT.cloned().unwrap();
        client
            .search(&input)
            .await
            .map_err(|err| CapturedError::new(err))
    });

    let empty = rsx! {
                div {
                    class: "search-empty",
                    span {
                        class: "text-primary",
                        "What do you want to listen to?"
                    }
                    span {
                        class: "text-secondary",
                        "Search for artists, albums, songs, or playlists"
                    }
                }
    };

    let content = match search_results.value() {
        Some(Ok(search_results)) if search_results.read().is_empty() && input().is_empty() => empty,
        None if input().is_empty() => empty,
        Some(Ok(search_results)) => rsx! {
            SearchResults {
                search_results
            }
        },
        Some(Err(err)) => rsx! {
            div {
                class: "col search-results",
                "Error: {err}"
            }
        },
        None => rsx! {
            div { }
        },
    };

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div {
            class: "search-view",
            div {
                class: "search-input-container row",
                SearchIcon { size: 18, filled: false }
                input {
                    id: "search-input",
                    type: "search",
                    placeholder: "Search...",
                    oninput: move |event| {
                        input.set(event.value());
                        search_results.call(event.value())
                    }
                }
            }
            {content}
        }
    }
}

#[component]
pub fn SearchResults(search_results: ReadSignal<Vec<SearchResult>>) -> Element {
    rsx! {
        div {
            class: "col search-results",
            for result in search_results.read().iter() {
                if let SearchResult::Artist {id, name, thumbnail } = result {
                    div {
                        class: "row",
                        if let Some(src) = thumbnail {
                            RoundedThumbnail {size: 36, artist: name, src }
                        }
                        div {
                            class: "col",
                            span { class: "text-primary", "{name}" }
                            span { class: "text-secondary", "Artist" }
                        }
                    }
                }
                else if let SearchResult::Album { id, name, cover, artist} = result {
                    div {
                        class: "row",
                        if let Some(src) = cover  {
                            Thumbnail { size: 36, artist: name, src }
                        }
                        div {
                            class: "col",
                            span {  class: "text-primary", "{name}" }
                            if let Some(artist) = artist {
                                span {class: "text-secondary", "Album · {artist}" }
                            } else {
                                span {class: "text-secondary", "Album" }
                            }
                        }
                    }
                }
                else if let SearchResult::Song {id, name, cover, artist } = result {
                    div {
                        class: "row",
                        if let Some(src) = cover  {
                            Thumbnail { size: 36, artist: name, src }
                        }
                        div {
                            class: "col",
                            span { class: "text-primary", "{name}" }
                            if let Some(artist) = artist {
                                span {class: "text-secondary", "Song · {artist}" }
                            } else {
                                span {class: "text-secondary", "Song" }
                            }
                        }
                    }
                }
            }
        }
    }
}
