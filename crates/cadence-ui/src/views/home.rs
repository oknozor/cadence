use std::time::Duration;

use crate::components::{
    AlbumList, AlbumMenuModal, PlaylistList, RadioStationList, TrackListWithCover, VerticalScroller,
};
use crate::navigation::topbar::{TopBar, TopBarFilter};
use crate::views::Route;
use cadence_core::hooks::{
    use_all_playlist, use_internet_radio_stations, use_random_songs, use_recently_played,
    use_recently_released,
};
use cadence_core::model::RadioStation;
use cadence_core::state::{CONTROLLER, ControllerExt};
use dioxus::prelude::*;
use dioxus_sdk::time::{TimeoutHandle, use_timeout};

#[component]
pub fn Home() -> Element {
    let nav = navigator();
    let mut controller = CONTROLLER.resolve();
    let active_filter = use_signal(|| TopBarFilter::All);
    let recently_released = use_recently_released();
    let recently_played = use_recently_played();
    let playlists = use_all_playlist();
    let random_songs = use_random_songs(5);
    let radio_stations = use_internet_radio_stations();
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

    let on_playlist_clicked = move |_playlist_id: String| tracing::debug!("unimplemented");
    let on_radio_station_clicked = move |radio: RadioStation| {
        info!(
            "Playing radio station with stream URL: {}",
            radio.stream_url
        );
        controller.play_radio(radio);
    };

    // If the radio filter is active, show radio stations view
    if active_filter() == TopBarFilter::Radio {
        let stations = match radio_stations() {
            Some(Ok(stations)) => stations,
            _ => {
                return rsx! {
                    TopBar { active_filter }
                    div { class: "music-content", "Loading radio stations..." }
                };
            }
        };

        return rsx! {
            TopBar { active_filter }
            VerticalScroller {
                div { class: "music-content",
                    RadioStationList {
                        title: "Radio Stations",
                        stations,
                        on_click: on_radio_station_clicked,
                    }
                }
            }
        };
    }

    // Normal home view
    let recently_released = match recently_released() {
        Some(Ok(recently_released)) => recently_released,
        _ => {
            return rsx! {
                TopBar { active_filter }
            };
        }
    };

    let recently_played = match recently_played() {
        Some(Ok(recently_played)) => recently_played,
        _ => {
            return rsx! {
                TopBar { active_filter }
            };
        }
    };

    let playlists = match playlists() {
        Some(Ok(playlists)) => playlists,
        _ => {
            return rsx! {
                TopBar { active_filter }
            };
        }
    };

    let random_songs = match random_songs() {
        Some(Ok(songs)) => songs,
        _ => {
            return rsx! {
                TopBar { active_filter }
            };
        }
    };

    rsx! {
        TopBar { active_filter }
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
