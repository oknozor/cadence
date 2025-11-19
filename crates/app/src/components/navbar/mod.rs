use std::time::Duration;

use dioxus::prelude::*;

use crate::{
    Route,
    components::icons::{home::HomeIcon, library::LibraryIcon, plus::PlusIcon, search::SearchIcon},
};

#[component]
pub fn Navbar() -> Element {
    let mut search_active = use_signal(|| false);
    let mut plus_active = use_signal(|| false);
    let mut home_active = use_signal(|| false);
    let mut library_active = use_signal(|| false);

    let mut set_active = move |active_signal: &mut Signal<bool>| {
        search_active.set(false);
        plus_active.set(false);
        home_active.set(false);
        library_active.set(false);
        active_signal.set(true);
    };

    let nav = navigator();

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div {
            id: "navbar",
            NavbarItem {
                label: "Home".to_string(),
                active: home_active,
                onclick: move || {
                    nav.replace(Route::Library {});
                    set_active(&mut home_active);
                },
                HomeIcon { filled: home_active }
            }
            NavbarItem {
                label: "Search".to_string(),
                active: search_active,
                onclick: move || {
                    nav.replace(Route::SearchView {});
                    set_active(&mut search_active);
                },
                SearchIcon { size: 32, filled: search_active }
            }
            NavbarItem {
                label: "Library".to_string(),
                active: library_active,
                onclick: move || {
                    set_active(&mut library_active);
                },
                LibraryIcon { filled: library_active }
            }
            NavbarItem {
                label: "Create".to_string(),
                active: plus_active,
                onclick: move || {
                    set_active(&mut plus_active);
                },
                PlusIcon { filled: plus_active }
            }
        }
    }
}

#[component]
pub fn NavbarItem(
    label: String,
    active: Signal<bool>,
    onclick: EventHandler<()>,
    children: Element,
) -> Element {
    let mut animate = use_signal(|| false);
    rsx! {
        div {
            class: "navbar-item",
            onclick: move |_| {
                active.set(!active());
                animate.set(true);
                spawn(async move {
                   dioxus_sdk::time::sleep(Duration::from_millis(200)).await;
                   animate.set(false);
                });

                onclick.call(())
            },
            div {
                class: if animate() { "navbar-item-icon active" } else { "navbar-item-icon" },
                {children}
            },
            div {
                class: "navbar-item-label",
                "{label}"
            }
        }
    }
}
