use cadence_core::model::SearchResult;
use dioxus::prelude::*;

use crate::{
    items::ItemInfo,
    thumbnails::{RoundedThumbnail, Thumbnail},
};

#[component]
pub fn SearchResults(search_results: ReadSignal<Vec<SearchResult>>) -> Element {
    rsx! {
        div { class: "search-results",
            for result in search_results.read().iter().cloned() {
                if let SearchResult::Artist { id, name, thumbnail } = result {
                    ArtistItemInfo {  name, thumbnail }
                } else if let SearchResult::Album { id, name, cover, artist } = result {
                    AlbumItemInfo {  name, cover, artist }
                } else if let SearchResult::Song { id, name, cover, artist } = result {
                    SongItemInfo {  name, cover, artist }
                }
            }
        }
    }
}

#[component]
pub fn ArtistItemInfo(name: ReadSignal<String>, thumbnail: Option<String>) -> Element {
    rsx! {
        div { class: "search-item",
            if let Some(src) = thumbnail {
                RoundedThumbnail { size: 50, name, src }
            }

            ItemInfo { primary: "{name}", secondary: "Artist" }
        }
    }
}

#[component]
pub fn AlbumItemInfo(
    name: ReadSignal<String>,
    artist: Option<String>,
    cover: Option<String>,
) -> Element {
    rsx! {
        div { class: "search-item",
            if let Some(src) = cover {
                Thumbnail { size: 50, name, src }
            }
            if let Some(artist) = artist {
                ItemInfo {
                    primary: name.clone(),
                    secondary: "Album · {artist}",
                }
            } else {
                ItemInfo { primary: name.clone(), secondary: "Album" }
            }
        }
    }
}

#[component]
pub fn SongItemInfo(
    name: ReadSignal<String>,
    artist: Option<String>,
    cover: Option<String>,
) -> Element {
    rsx! {
        div { class: "search-item",
            if let Some(src) = cover {
                Thumbnail { size: 50, name, src }
            }
            if let Some(artist) = artist {
                ItemInfo {
                    primary: name.clone(),
                    secondary: "Song · {artist}",
                }
            } else {
                ItemInfo { primary: name.clone(), secondary: "Song" }
            }
        }
    }
}
