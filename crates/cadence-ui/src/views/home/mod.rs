use crate::components::{AlbumList, PlaylistList};
use crate::{navigation::topbar::TopBar, views::Route};
use cadence_core::hooks::{use_all_playlist, use_recently_played, use_recently_released};
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let nav = navigator();
    let recently_released = use_recently_released();
    let recently_played = use_recently_played();
    let playlists = use_all_playlist();

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

    let playlists = match playlists() {
        Some(Ok(playlists)) => playlists,
        _ => {
            return rsx!(
                p { "Failed to load playlists" }
            );
        }
    };

    let on_album_clicked = move |album_id| {
        nav.push(Route::AlbumView { id: album_id });
    };

    let on_playlist_clicked = move |_playlist_id: String| tracing::debug!("unimplemented");

    rsx! {
        TopBar {}
        div { class: "music-content",
            AlbumList {
                title: "Recently Played",
                albums: recently_played,
                on_card_clicked: on_album_clicked,
            }

            AlbumList {
                title: "Recently Released",
                albums: recently_released,
                on_card_clicked: on_album_clicked,
            }

            PlaylistList {
                title: "Playlists",
                playlists: playlists,
                on_card_clicked: on_playlist_clicked,
            }
        }
    }
}
