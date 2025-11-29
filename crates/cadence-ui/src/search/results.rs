use cadence_core::{
    model::{SearchResult, Song},
    state::{CONTROLLER, ControllerExt, ControllerStoreExt},
};
use dioxus::prelude::*;

use crate::{
    icons::{dots::DotIcon, plus::PlusIcon},
    items::ItemInfo,
    thumbnails::{RoundedThumbnail, Thumbnail},
    views::Route,
};

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

#[component]
pub fn SearchResultRow(
    thumbnail: Option<Element>,
    content: Element,
    action: Option<Element>,
    callback: EventHandler,
) -> Element {
    rsx! {
        div { class: "search-item",
            div {
                class: "search-item-start",
                onclick: move |_| {
                    callback.call(());
                },
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
pub fn ArtistItemInfo(
    id: ReadSignal<String>,
    name: ReadSignal<String>,
    thumbnail: Option<String>,
) -> Element {
    let thumbnail = thumbnail.map(|src| {
        rsx! {
            RoundedThumbnail { size: 50, name, src }
        }
    });

    let content = rsx! {
        ItemInfo { primary: "{name}", secondary: "Artist" }
    };

    let callback = |_| ();

    rsx! {
        SearchResultRow {
            thumbnail,
            content,
            action: None,
            callback,
        }
    }
}

#[component]
pub fn AlbumItemInfo(
    id: ReadSignal<String>,
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

    let callback = move |_| {
        navigator().push(Route::AlbumView {
            id: id.read().clone(),
        });
    };

    rsx! {
        SearchResultRow {
            thumbnail,
            content,
            action,
            callback,
        }

    }
}

#[component]
pub fn SongItemInfo(song: Song) -> Element {
    let mut controller = CONTROLLER.resolve();
    let active = controller.is_active(&song.id);
    let paused = !*controller.is_playing().read();

    let Song {
        id,
        title,
        artist,
        cover_art,
        ..
    } = song.clone();

    let thumbnail = cover_art.map(|src| {
        rsx! {
            Thumbnail { size: 50, name: title.clone(), src }
        }
    });

    let content = rsx! {
        ItemInfo {
            primary: title,
            secondary: "Song · {artist}",
            active,
            paused,
        }
    };

    let action = Some(rsx! {
        DotIcon { size: 18 }
        PlusIcon { size: 18, filled: false }
    });

    let callback = move || controller.queue_now(song.clone());

    rsx! {
        SearchResultRow {
            thumbnail,
            content,
            action,
            callback,
        }
    }
}
