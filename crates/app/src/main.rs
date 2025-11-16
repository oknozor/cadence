use crate::context::{IsPlaying, Queue, SubSonicLogin};
use cadence_player::CadencePlayer;
use components::{login::Login, navbar::Navbar, player::Player};
use dioxus::prelude::*;
use dioxus_sdk::storage::{LocalStorage, get_from_storage, use_storage};
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
const COMPONENT_CSS: Asset = asset!("/assets/dx-components-theme.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    tracing::info!("Candence started");
    let mut logged_in = use_signal(|| false);
    #[cfg(not(feature = "mobile"))]
    let mut saved_credentials = get_from_storage::<LocalStorage, Option<SubSonicLogin>>(
        "subsonic_credentials".to_string(),
        || None,
    );

    #[cfg(not(feature = "mobile"))]
    if let Some(credentials) = saved_credentials.take() {
        logged_in.set(true);
        *SUBSONIC_CLIENT.write() = Some(SubsonicClient::new(
            &credentials.server_url,
            &credentials.username,
            &credentials.password,
        ));
    }

    #[cfg(not(feature = "mobile"))]
    let mut saved_credentials =
        use_storage::<LocalStorage, _>("subsonic_credentials".to_string(), || saved_credentials);

    let (tx, rx) = tokio::sync::mpsc::channel(10);
    let (position_tx, _) = tokio::sync::broadcast::channel(10);
    let rx = use_signal(|| Arc::new(Mutex::new(rx)));
    let _: broadcast::Sender<u64> = use_context_provider(|| position_tx);
    let _ = use_context_provider(|| tx);
    let _ = use_context_provider(Queue::default);
    let _ = use_context_provider(IsPlaying::default);
    let mut error_msg = use_signal(|| None::<String>);

    let handle_login = {
        move |(server_url, username, password): (String, String, String)| {
            let subsonic_client = SubsonicClient::new(&server_url, &username, &password);

            #[cfg(not(feature = "mobile"))]
            saved_credentials.set(Some(SubSonicLogin {
                server_url: server_url.clone(),
                username: username.clone(),
                password: password.clone(),
            }));

            *SUBSONIC_CLIENT.write() = Some(subsonic_client.clone());
            spawn(async move {
                match subsonic_client.ping().await {
                    Ok(_) => {
                        spawn(async move {
                            let mut player = CadencePlayer::build(
                                &server_url,
                                &username,
                                &password,
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
    };

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: COMPONENT_CSS }
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
        Navbar {}
        Outlet::<Route> {}
    }
}
