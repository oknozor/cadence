use dioxus::prelude::*;

use crate::{
    components::{ItemInfo, ItemRow, RoundedThumbnail},
    views::Route,
};

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

    let callback = move |_| {
        navigator().push(Route::ArtistView {
            id: id.read().clone(),
        });
    };

    rsx! {
        ItemRow {
            thumbnail,
            content,
            action: None,
            callback,
        }
    }
}
