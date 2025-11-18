use dioxus::prelude::*;
use crate::components::icons::gear::GearIcon;
use crate::Route;

#[component]
pub fn TopBar() -> Element {
    let nav = navigator();

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div {
        id: "topbar",
            div {
                class: "settings-icons",
                GearIcon {
                    filled: false,
                }
            }
            button {
                onclick: move |_| {
                    nav.replace(Route::Library {});
                },
                "Music"
            }
            button {
                onclick: move |_| {
                    nav.replace(Route::Library {});
                },
                "Podcasts"
            }
            button {
                onclick: move |_| {
                    nav.replace(Route::Library {});
                },
                "Radio"
            }
        }
    }
}