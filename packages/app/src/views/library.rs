//! Library view for browsing music

use std::future::Future;

use dioxus::{CapturedError, prelude::*};

use ui::{
    Album, AlbumList, Player, PlayerTrack,
    client::{AlbumListType, SUBSONIC_CLIENT},
};

use crate::Route;

#[component]
pub fn Library() -> Element {
    let nav = navigator();
    let current_track = use_signal(|| None::<PlayerTrack>);

    let recently_released = use_resource(|| fetch_albums(AlbumListType::RecentlyReleased));
    let recently_played = use_resource(|| fetch_albums(AlbumListType::RecentlyPlayed));

    let recently_released = match recently_released() {
        Some(Ok(recently_released)) => recently_released,
        _ => {
            return rsx!(p { "Failed to load recently released albums" });
        }
    };

    let recently_played = match recently_played() {
        Some(Ok(recently_played)) => recently_played,
        _ => {
            return rsx!(p { "Failed to load recently played albums" });
        }
    };

    rsx! {
        div {
            class: "library-view",
            div {
                class: "music-content",
                AlbumList {
                    title: "Recently Played",
                    albums: recently_played,
                    on_album_select: move |album_id| {
                        nav.push(Route::AlbumView { id: album_id });
                    }
                }

                AlbumList {
                    title: "Recently Released",
                    albums: recently_released,
                    on_album_select: move |album_id| {
                        nav.push(Route::AlbumView { id: album_id });
                    }
                }
            }

            div {
                class: "player-section",
                Player {
                    current_track: current_track.read().clone(),
                    on_play: || {},
                    on_pause: || {},
                    on_next: || {},
                    on_previous: || {},
                }
            }
        }
    }
}

fn fetch_albums(
    album_type: AlbumListType,
) -> impl Future<Output = dioxus::Result<Vec<Album>, CapturedError>> {
    async move {
        let response = SUBSONIC_CLIENT()
            .unwrap()
            .list_album(album_type)
            .await
            .map_err(|err| CapturedError::from_display(format!("{err}")))?;
        Ok(response)
    }
}
