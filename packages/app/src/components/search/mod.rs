use dioxus::prelude::*;

pub enum SearchResult {
    Artist {
        id: String,
        name: String,
    },
    Album {
        id: String,
        name: String,
    },
    Song {
        id: String,
        name: String,
    }
}

pub fn SearchResultComponent() -> Element {
    rsx! {

    }
}