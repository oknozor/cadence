use cadence_core::hooks::init_global_context;
use cadence_core::hooks::use_login_state;
use cadence_core::{hooks::use_saved_credentials, state::SubSonicLogin};
use cadence_ui::UI_CSS;
use cadence_ui::login::Login;
use cadence_ui::player::Player;
use dioxus::prelude::*;
use navigation::navbar::Navbar;
use views::{AlbumView, Home, LibraryView, SearchView};

mod navigation;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(WebNavbar)]
    #[route("/")]
    Home { },
    #[route("/search")]
    SearchView { },
    #[route("/library")]
    LibraryView { },
    #[route("/album/:id")]
    AlbumView { id: String },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styles/main.scss");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    #[cfg(feature = "desktop")]
    dioxus_sdk::storage::set_dir("~/.local/share/cadence");

    #[cfg(feature = "mobile")]
    let dir = cadence_storage_android::internal_storage_dir();

    info!("Candence started");
    init_global_context();

    let mut saved_credentials = use_saved_credentials();
    let login_state = use_login_state();

    let handle_login = move |(server_url, username, password): (String, String, String)| {
        saved_credentials.set(Some(SubSonicLogin {
            server_url: server_url.clone(),
            username: username.clone(),
            password: password.clone(),
        }));
    };

    rsx! {
        document::Meta {
            name: "viewport",
            content: "width=device-width, initial-scale=1.0, viewport-fit=cover",
        }
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: UI_CSS }
        document::Script {
            r#type: "module",
            src: asset!("/assets/howler.min.js", JsAssetOptions::new().with_minify(true)),
        }
        meta {
            content: "width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no, viewport-fit=cover",
            name: "viewport",
        }
        if *login_state.logged_in().read() {
            Router::<Route> {}
            Player {}
        } else {
            Login { on_login: handle_login }
            if let Some(err) = login_state.errored().read().as_ref() {
                div { class: "error", "{err}" }
            }
        }
    }
}

#[component]
fn WebNavbar() -> Element {
    rsx! {
        Outlet::<Route> {}
        Navbar {}
    }
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn Java_dev_dioxus_main_MainActivity_notifyOnNewIntent(
    mut env: jni::JNIEnv,
    _class: jni::objects::JClass,
    data: jni::objects::JString,
) {
    let string: String = match env.get_string(&data) {
        Ok(s) => s.into(),
        Err(e) => {
            eprintln!("Failed to convert Java string: {:?}", e);
            return;
        }
    };
    error!("New intent received: {}", string);
}
