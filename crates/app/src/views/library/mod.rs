use dioxus::{CapturedError, prelude::*};

use crate::{
    Route,
    components::{album_card::Album, album_list::AlbumList, topbar::TopBar},
    services::subsonic_client::AlbumListType,
};

use crate::services::subsonic_client::SUBSONIC_CLIENT;

#[component]
pub fn Library() -> Element {
    let nav = navigator();

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
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div {
            class: "library-view",
            TopBar {  }
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
        }
    }
}

async fn fetch_albums(album_type: AlbumListType) -> dioxus::Result<Vec<Album>, CapturedError> {
    let response = SUBSONIC_CLIENT()
        .unwrap()
        .list_album(album_type)
        .await
        .map_err(|err| CapturedError::from_display(format!("{err}")))?;
    Ok(response)
}
