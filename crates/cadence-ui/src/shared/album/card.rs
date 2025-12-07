use crate::shared::ItemInfo;
use cadence_core::model::Album;
use dioxus::prelude::*;

#[component]
pub fn AlbumCard(album: Album, on_album_select: EventHandler<String>) -> Element {
    rsx! {
        div {
            class: "album-card",
            key: "{album.id}",
            onclick: move |_| on_album_select.call(album.id.clone()),

            if let Some(cover) = album.cover_art.as_ref() {
                img {
                    src: "{cover}",
                    alt: "{album.name}",
                    width: "100px",
                    height: "100px",
                }
            } else {
                div { class: "no-cover", "🎵" }
            }

            ItemInfo {
                primary: album.name,
                secondary: album.artist,
                is_active: false,
                is_paused: false,
            }
        }
    }
}
