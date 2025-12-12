use crate::{
    components::{ItemInfo, ItemRow, PlusIcon, Thumbnail},
    views::Route,
};
use dioxus::prelude::*;

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
                ItemInfo { primary: name, secondary: "Album · {artist}" }
            }
        })
        .unwrap_or(rsx! {
            ItemInfo { primary: name, secondary: "Album" }
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
        ItemRow {
            thumbnail,
            content,
            action,
            callback,
        }
    }
}
