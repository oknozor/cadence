use dioxus::prelude::*;

use crate::icons::{home::HomeIcon, library::LibraryIcon, plus::PlusIcon, search::SearchIcon};

const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");

#[component]
pub fn Navbar() -> Element {
    let search_active = use_signal(|| false);
    let plus_active = use_signal(|| false);
    let home_active = use_signal(|| false);
    let library_active = use_signal(|| false);

    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }

        div {
            id: "navbar",
            NavbarItem {
                label: "Home".to_string(),
                active: home_active,
                onclick: || {},
                HomeIcon { filled: home_active }
            }
            NavbarItem {
                label: "Search".to_string(),
                active: search_active,
                onclick: || {},
                SearchIcon { filled: search_active }
            }
            NavbarItem {
                label: "Library".to_string(),
                active: library_active,
                onclick: || {},
                LibraryIcon { filled: library_active }
            }
            NavbarItem {
                label: "Create".to_string(),
                active: plus_active,
                onclick: || {},
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
    rsx! {
        div {
            class: "navbar-item",
            onclick: move |_| {
                active.set(!active());
                onclick.call(())
            },
            div {
                class: "navbar-item-icon",
                {children}
            },
            div {
                class: "navbar-item-label",
                "{label}"
            }
        }
    }
}
