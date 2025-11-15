use cadence_player::CadencePlayer;
use dioxus::prelude::*;
use std::sync::Arc;
use tokio::sync::{Mutex, broadcast};
use ui::{
    AlbumView, IsPlaying, Login, Navbar, Player, Queue, SubsonicClient, client::SUBSONIC_CLIENT,
};
use views::Library;

mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(WebNavbar)]
    #[route("/")]
    Library { },
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
    let (tx, rx) = tokio::sync::mpsc::channel(10);
    let (position_tx, _) = tokio::sync::broadcast::channel(10);
    let rx = use_signal(|| Arc::new(Mutex::new(rx)));
    let _: broadcast::Sender<u64> = use_context_provider(|| position_tx);
    let _ = use_context_provider(|| tx);
    let _ = use_context_provider(Queue::default);
    let _ = use_context_provider(IsPlaying::default);
    let mut logged_in = use_signal(|| false);
    let mut error_msg = use_signal(|| None::<String>);

    let handle_login = {
        move |(server_url, username, password): (String, String, String)| {
            let subsonic_client = SubsonicClient::new(&server_url, &username, &password);
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

/// A web-specific Router around the shared `Navbar` component
/// which allows us to use the web-specific `Route` enum.
#[component]
fn WebNavbar() -> Element {
    rsx! {
        Navbar {}
        Outlet::<Route> {}
    }
}
