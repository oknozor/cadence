use crate::components::{
    AudioIdentificationIcon, HomeIcon, LibraryIcon, Player, PlusIcon, RadioPlayer, SearchIcon,
};
use crate::views::Route;
use cadence_core::state::{CONTROLLER, ControllerStoreExt};
use dioxus::prelude::*;
use std::time::Duration;

#[component]
pub fn Navbar() -> Element {
    let mut search_active = use_signal(|| false);
    let mut plus_active = use_signal(|| false);
    let mut listen_active = use_signal(|| false);
    let mut home_active = use_signal(|| false);
    let mut library_active = use_signal(|| false);

    let mut set_active = move |active_signal: &mut Signal<bool>| {
        search_active.set(false);
        plus_active.set(false);
        home_active.set(false);
        library_active.set(false);
        listen_active.set(false);
        active_signal.set(true);
    };

    let nav = navigator();

    let controller = CONTROLLER.resolve();
    let is_radio_playing = controller.current_radio().read().is_some();

    rsx! {
        div { class: "navbar-container",
            if is_radio_playing {
                RadioPlayer {}
            } else {
                Player {}
            }
            div { id: "navbar",
                NavbarItem {
                    label: "Home".to_string(),
                    active: home_active,
                    onclick: move || {
                        nav.replace(Route::Home {});
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
                        nav.replace(Route::LibraryView {});
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
                    PlusIcon { size: 32, filled: plus_active }
                }
                if cfg!(target_os = "android") {
                    NavbarItem {
                        label: "Listen".to_string(),
                        active: listen_active,
                        onclick: move || {
                            #[cfg(target_os = "android")]
                            nav.replace(Route::ShazamView {});
                            set_active(&mut listen_active);
                        },
                        AudioIdentificationIcon { size: 32, filled: listen_active }
                    }
                }
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
            div { class: if animate() { "navbar-item-icon active" } else { "navbar-item-icon" }, {children} }
            div { class: "navbar-item-label", "{label}" }
        }
    }
}
