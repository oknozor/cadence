use std::time::Duration;

use crate::components::{
    AlbumList, AlbumMenuModal, PlaylistList, TrackListWithCover, VerticalScroller,
};
use crate::{navigation::topbar::TopBar, views::Route};
use cadence_core::hooks::{
    use_all_playlist, use_random_songs, use_recently_played, use_recently_released,
};
use dioxus::prelude::*;
use dioxus_sdk::time::{TimeoutHandle, use_timeout};

#[component]
pub fn Home() -> Element {
    let nav = navigator();
    let recently_released = use_recently_released();
    let recently_played = use_recently_played();
    let playlists = use_all_playlist();
    let random_songs = use_random_songs(5);
    let mut album_modal_open = use_signal(|| false);
    let mut album_selected = use_signal(|| None);
    let mut album_modal_timeout_handle: Signal<Option<TimeoutHandle>> = use_signal(|| None);
    let album_modal_timeout = use_timeout(Duration::from_secs(1), move |album| {
        album_modal_timeout_handle.set(None);
        album_selected.set(Some(album));
        album_modal_open.set(true);
    });

    let on_album_pressed = move |album| {
        album_modal_timeout.action(album);
    };

    let on_album_clicked = move |album_id| {
        if let Some(handle) = *album_modal_timeout_handle.read() {
            handle.cancel();
        }
        nav.push(Route::AlbumView { id: album_id });
    };

    let recently_released = match recently_released() {
        Some(Ok(recently_released)) => recently_released,
        _ => {
            return rsx!();
        }
    };

    let recently_played = match recently_played() {
        Some(Ok(recently_played)) => recently_played,
        _ => {
            return rsx!();
        }
    };

    let playlists = match playlists() {
        Some(Ok(playlists)) => playlists,
        _ => {
            return rsx!();
        }
    };

    let random_songs = match random_songs() {
        Some(Ok(songs)) => songs,
        _ => {
            return rsx!();
        }
    };

    let on_playlist_clicked = move |_playlist_id: String| tracing::debug!("unimplemented");

    rsx! {
        TopBar {}
        VerticalScroller {

            div { class: "music-content",
                AlbumList {
                    title: "Recently Played",
                    albums: recently_played,
                    on_press: on_album_pressed,
                    on_click: on_album_clicked,
                }

                AlbumList {
                    title: "Recently Released",
                    albums: recently_released,
                    on_click: on_album_clicked,
                    on_press: on_album_pressed,
                }

                TrackListWithCover { title: "Play now", songs: random_songs }

                PlaylistList {
                    title: "Playlists",
                    playlists,
                    on_card_clicked: on_playlist_clicked,
                }
            }
        }
        if let Some(album) = album_selected() {
            AlbumMenuModal { open: album_modal_open, album }
        }
    }
}
