use cadence_core::hooks::effects::use_audio_backend;
use cadence_core::hooks::effects::use_audio_backend_state_update;
use cadence_core::hooks::effects::use_notification_control;
use cadence_core::hooks::init_global_context;
use cadence_core::player::NotificationControl;
use cadence_core::{hooks::use_saved_credentials, state::SubSonicLogin};
use cadence_ui::UI_CSS;
use cadence_ui::login::Login;
use cadence_ui::views::Route;
use dioxus::prelude::*;

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

    init_global_context();
    NotificationControl::init(consume_context());
    use_notification_control();

    let mut saved_credentials = use_saved_credentials();
    let mut ready = use_signal(|| false);
    let logged_in = use_signal(|| false);
    use_audio_backend(ready, logged_in);
    use_audio_backend_state_update();

    let handle_login = move |(server_url, username, password): (String, String, String)| {
        info!("Saving credentials");
        saved_credentials.set(Some(SubSonicLogin {
            server_url: server_url.clone(),
            username: username.clone(),
            password: password.clone(),
        }));
        ready.set(true);
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
        if *logged_in.read() {
            Router::<Route> {}
        } else {
            Login { on_login: handle_login }
        }
    }
}
