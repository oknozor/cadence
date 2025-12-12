use crate::components::{AlbumItemInfo, ArtistItemInfo, SongItemInfo};
use cadence_core::model::SearchResult;
use dioxus::prelude::*;

#[component]
pub fn SearchResults(search_results: ReadSignal<Vec<SearchResult>>) -> Element {
    rsx! {
        div { class: "search-results",
            for result in search_results.read().iter().cloned() {
                if let SearchResult::Artist { id, name, thumbnail } = result {
                    ArtistItemInfo { id, name, thumbnail }
                } else if let SearchResult::Album { id, name, cover, artist } = result {
                    AlbumItemInfo {
                        id,
                        name,
                        cover,
                        artist,
                    }
                } else if let SearchResult::Song(song) = result {
                    SongItemInfo { song }
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
