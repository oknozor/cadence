use std::time::Duration;
use dioxus::prelude::*;
use crate::components::icons::gear::GearIcon;
use crate::components::icons::home::HomeIcon;
use crate::Route;

#[component]
pub fn TopBar() -> Element {
    let nav = navigator();

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div{
            id: "topbar",
            div {
                id: "header",
                div {
                    class: "cadence-icon",
                    HomeIcon {
                       filled: false
                    }
                }
                div {
                    class: "settings-icons",
                    GearIcon {
                        filled: false,
                    }
                }
            }
            div {
                id: "topbar-source-selector",
                div {
                  button {
                        onclick: move |_| {
                            nav.replace(Route::Library {});
                        },
                        "Music"
                    }
                }
                div {
                    button {
                        onclick: move |_| {
                            nav.replace(Route::Library {});
                        },
                        "Podcasts"
                    }
                }
                div {
                    button {
                        onclick: move |_| {
                            nav.replace(Route::Library {});
                        },
                        "Radio"
                    }
                }
            }
        }
    }
}