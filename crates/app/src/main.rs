use crate::context::{IsPlaying, Queue, SubSonicLogin};
use cadence_player::CadencePlayer;
use components::{login::Login, navbar::Navbar, player::Player};
use dioxus::prelude::*;
use dioxus_sdk::storage::{get_from_storage, use_storage};

#[cfg(feature = "mobile")]
use cadence_storage_android::LocalStorage;

#[cfg(not(feature = "mobile"))]
use dioxus_sdk::storage::LocalStorage;

use crate::components::topbar::TopBar;
use services::subsonic_client::{SUBSONIC_CLIENT, SubsonicClient};
use std::sync::Arc;
use tokio::sync::{Mutex, broadcast};
use views::{AlbumView, Library, SearchView};

mod components;
mod context;
mod services;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(WebNavbar)]
    #[route("/")]
    Library { },
    #[route("/search")]
    SearchView { },
    #[route("/album/:id")]
    AlbumView { id: String },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    #[cfg(feature = "desktop")]
    dioxus_sdk::storage::set_dir!("/home/okno/.local/share/cadence");

    #[cfg(feature = "mobile")]
    let dir = cadence_storage_android::internal_storage_dir();

    info!("Candence started");
    let mut logged_in = use_signal(|| false);
    let mut error_msg = use_signal(|| None::<String>);
    let (tx, rx) = tokio::sync::mpsc::channel(10);
    let (position_tx, _) = tokio::sync::broadcast::channel(10);
    let rx = use_signal(|| Arc::new(Mutex::new(rx)));
    let _: broadcast::Sender<u64> = use_context_provider(|| position_tx);
    let _ = use_context_provider(|| tx);
    let _ = use_context_provider(Queue::default);
    let _ = use_context_provider(IsPlaying::default);

    let mut saved_credentials = {
        let saved = get_from_storage::<LocalStorage, Option<SubSonicLogin>>(
            "subsonic_credentials".to_string(),
            || None,
        );

        use_storage::<LocalStorage, _>("subsonic_credentials".to_string(), || saved)
    };

    use_effect(move || {
        if let Some(credentials) = saved_credentials.read().cloned() {
            logged_in.set(true);

            let client = SubsonicClient::new(
                &credentials.server_url,
                &credentials.username,
                &credentials.password,
            );
            *SUBSONIC_CLIENT.write() = Some(client.clone());

            spawn(async move {
                match client.ping().await {
                    Ok(_) => {
                        spawn(async move {
                            let mut player = CadencePlayer::build(
                                &credentials.server_url,
                                &credentials.username,
                                &credentials.password,
                                rx.read().clone(),
                                consume_context(),
                            )
                            .expect("Player start failed");
                            player.run().await.expect("Player run error");
                        });
                        logged_in.set(true);
                    }
                    Err(err) => {
                        error_msg.set(Some(format!("Login failed: {}", err)));
                    }
                }
            });
        }
    });

    let handle_login = {
        move |(server_url, username, password): (String, String, String)| {
            saved_credentials.set(Some(SubSonicLogin {
                server_url: server_url.clone(),
                username: username.clone(),
                password: password.clone(),
            }));
        }
    };

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Script {
            type: "module",
            src: asset!("/assets/howler.min.js", JsAssetOptions::new().with_minify(true)),
        }
        if logged_in() {
            Router::<Route> { }
            Player {}
        } else {
            Login {
                on_login: handle_login,
            }
            if let Some(err) = error_msg() {
                div { class: "error", "{err}" }
            }
        }
    }
}

#[component]
fn WebNavbar() -> Element {
    rsx! {
        TopBar {}
        Outlet::<Route> {}
        Navbar {}
    }
}
