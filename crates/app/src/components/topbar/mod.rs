use crate::Route;
use crate::components::icons::gear::GearIcon;
use dioxus::prelude::*;

#[component]
pub fn TopBar() -> Element {
    let nav = navigator();

    rsx! {
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
                    nav.replace(Route::Home {});
                },
                "Music"
            }
            button {
                onclick: move |_| {
                    nav.replace(Route::Home {});
                },
                "Podcasts"
            }
            button {
                onclick: move |_| {
                    nav.replace(Route::Home {});
                },
                "Radio"
            }
        }
    }
}
