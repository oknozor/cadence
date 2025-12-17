use cadence_core::model::PlaylistInfo;
use dioxus::prelude::*;

use crate::components::ItemInfo;

#[component]
pub fn PlaylistCard(playlist: PlaylistInfo, on_card_clicked: EventHandler<String>) -> Element {
    rsx! {
        div {
            class: "playlist-card",
            key: "{playlist.id}",
            onclick: move |_| on_card_clicked.call(playlist.id.clone()),
            if let Some(cover) = playlist.cover_art.as_ref() {
                img {
                    src: "{cover}",
                    alt: "{playlist.name}",
                    width: "100px",
                    height: "100px",
                }
            } else {
                div { class: "no-cover", "🎵" }
            }

            ItemInfo {
                primary: playlist.name,
                secondary: playlist.owner.map(|owner| format!("By {owner}")).unwrap_or_default(),
                is_active: false,
                is_paused: false,
            }
        }
    }
}
