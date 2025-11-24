use cadence_core::model::SearchResult;
use dioxus::prelude::*;

use crate::{
    icons::{dots::DotIcon, plus::PlusIcon},
    items::ItemInfo,
    thumbnails::{RoundedThumbnail, Thumbnail},
};

#[component]
pub fn SearchResults(search_results: ReadSignal<Vec<SearchResult>>) -> Element {
    rsx! {
        div { class: "search-results",
            for result in search_results.read().iter().cloned() {
                if let SearchResult::Artist { id, name, thumbnail } = result {
                    ArtistItemInfo { name, thumbnail }
                } else if let SearchResult::Album { id, name, cover, artist } = result {
                    AlbumItemInfo { name, cover, artist }
                } else if let SearchResult::Song { id, name, cover, artist } = result {
                    SongItemInfo { name, cover, artist }
                }
            }
        }
    }
}

#[component]
pub fn SearchResultsEmpty() -> Element {
    rsx! {
        div { class: "search-empty",
            span { "What do you want to listen to?" }
            span { "Search for artists, albums, songs, or playlists" }
        }
    }
}

#[component]
pub fn SearchResultRow(
    thumbnail: Option<Element>,
    content: Element,
    action: Option<Element>,
) -> Element {
    rsx! {
        div { class: "search-item",
            div { class: "search-item-start",
                if let Some(thumbnail) = thumbnail {
                    {thumbnail}
                }
                {content}
            }
            if let Some(action) = action {
                div { class: "search-item-action", {action} }
            }
        }
    }
}

#[component]
pub fn ArtistItemInfo(name: ReadSignal<String>, thumbnail: Option<String>) -> Element {
    let thumbnail = thumbnail.map(|src| {
        rsx! {
            RoundedThumbnail { size: 50, name, src }
        }
    });
    let content = rsx! {
        ItemInfo { primary: "{name}", secondary: "Artist" }
    };
    rsx! {
        SearchResultRow { thumbnail, content, action: None }
    }
}

#[component]
pub fn AlbumItemInfo(
    name: ReadSignal<String>,
    artist: Option<String>,
    cover: Option<String>,
) -> Element {
    let thumbnail = cover.map(|src| {
        rsx! {
            Thumbnail { size: 50, name, src }
        }
    });
    let content = artist
        .map(|artist| {
            rsx! {
                ItemInfo { primary: name.clone(), secondary: "Album · {artist}" }
            }
        })
        .unwrap_or(rsx! {
            ItemInfo { primary: name.clone(), secondary: "Album" }
        });

    let action = Some(rsx! {
        PlusIcon { size: 18, filled: false }
    });

    rsx! {
        SearchResultRow { thumbnail, content, action }

    }
}

#[component]
pub fn SongItemInfo(
    name: ReadSignal<String>,
    artist: Option<String>,
    cover: Option<String>,
) -> Element {
    let thumbnail = cover.map(|src| {
        rsx! {
            Thumbnail { size: 50, name, src }
        }
    });
    let content = artist
        .map(|artist| {
            rsx! {
                ItemInfo { primary: name.clone(), secondary: "Song · {artist}" }
            }
        })
        .unwrap_or(rsx! {
            ItemInfo { primary: name.clone(), secondary: "Song" }
        });

    let action = Some(rsx! {
        DotIcon { size: 18 }
        PlusIcon { size: 18, filled: false }
    });

    rsx! {
        SearchResultRow { thumbnail, content, action }

    }
}
