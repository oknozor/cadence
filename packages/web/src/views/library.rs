//! Library view for browsing music

use dioxus::prelude::*;
use ui::{Album, LibraryBrowser, Login, Player, SubsonicClient, Track};

#[component]
pub fn Library() -> Element {
    let mut client = use_signal(|| None::<SubsonicClient>);
    let albums = use_signal(|| Vec::<Album>::new());
    let current_track = use_signal(|| None::<Track>);
    let mut error_msg = use_signal(|| None::<String>);

    let handle_login = move |(server_url, username, password): (String, String, String)| {
        spawn(async move {
            match SubsonicClient::new(&server_url, &username, &password) {
                Ok(subsonic_client) => {
                    match subsonic_client.ping().await {
                        Ok(_) => {
                            client.set(Some(subsonic_client));
                            error_msg.set(None);
                            // TODO: Fetch albums from the server
                        }
                        Err(e) => {
                            error_msg.set(Some(format!("Connection failed: {:?}", e)));
                        }
                    }
                }
                Err(e) => {
                    error_msg.set(Some(format!("Failed to create client: {:?}", e)));
                }
            }
        });
    };

    let handle_album_select = move |album_id: String| {
        // TODO: Load album tracks and play first track
        let _ = album_id; // Placeholder
    };

    let handle_play = move |_| {
        // TODO: Implement play functionality
    };

    let handle_pause = move |_| {
        // TODO: Implement pause functionality
    };

    let handle_next = move |_| {
        // TODO: Implement next track functionality
    };

    let handle_previous = move |_| {
        // TODO: Implement previous track functionality
    };

    rsx! {
        div {
            class: "library-view",

            if let Some(err) = error_msg.read().as_ref() {
                div {
                    class: "error-message",
                    "{err}"
                }
            }

            if client.read().is_none() {
                Login {
                    on_login: handle_login,
                }
            } else {
                div {
                    class: "music-content",
                    LibraryBrowser {
                        albums: albums.read().clone(),
                        on_album_select: handle_album_select,
                    }

                    div {
                        class: "player-section",
                        Player {
                            current_track: current_track.read().clone(),
                            on_play: handle_play,
                            on_pause: handle_pause,
                            on_next: handle_next,
                            on_previous: handle_previous,
                        }
                    }
                }
            }
        }
    }
}
