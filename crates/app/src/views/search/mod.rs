use crate::components::icons::search::SearchIcon;
use crate::shared::thumbnails::{RoundedThumbnail, Thumbnail};
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
            if search_results.read().is_empty() {
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
            } else {
                SearchResults { search_results }
            }
        }
    }
}

#[component]
pub fn SearchInput(search_results: WriteSignal<Vec<SearchResult>>) -> Element {
    let client = SUBSONIC_CLIENT.cloned().unwrap();

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div {
            class: "search-input-container row",
            SearchIcon { size: 18, filled: false }
            input {
                id: "search-input",
                type: "search",
                placeholder: "Search...",
                oninput: move |event| {
                    let client = client.clone();
                    spawn(async move {
                        match client.search(&event.value()).await {
                            Ok(results) => {
                                *search_results.write() = results;
                            }
                            Err(err) => {
                                error!("Search error: {}", err);
                            }
                        }
                    });
                }
            }
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
