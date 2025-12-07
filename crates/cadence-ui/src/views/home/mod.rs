use crate::shared::AlbumList;
use crate::{navigation::topbar::TopBar, views::Route};
use cadence_core::hooks::{use_recently_played, use_recently_released};
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let nav = navigator();

    let recently_released = use_recently_released();
    let recently_played = use_recently_played();

    let recently_released = match recently_released() {
        Some(Ok(recently_released)) => recently_released,
        _ => {
            return rsx!(
                p { "Failed to load recently released albums" }
            );
        }
    };

    let recently_played = match recently_played() {
        Some(Ok(recently_played)) => recently_played,
        _ => {
            return rsx!(
                p { "Failed to load recently played albums" }
            );
        }
    };

    rsx! {
        TopBar {}
        div { class: "music-content",
            AlbumList {
                title: "Recently Played",
                albums: recently_played,
                on_album_select: move |album_id| {
                    nav.push(Route::AlbumView { id: album_id });
                },
            }

            AlbumList {
                title: "Recently Released",
                albums: recently_released,
                on_album_select: move |album_id| {
                    nav.push(Route::AlbumView { id: album_id });
                },
            }
        }
    }
}
