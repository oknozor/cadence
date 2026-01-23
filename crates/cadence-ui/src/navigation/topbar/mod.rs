use crate::components::{CloseIcon, ExpandableButton, GearIcon, MenuButton};
use crate::views::Route;
use dioxus::prelude::*;

#[component]
pub fn TopBar() -> Element {
    let nav = navigator();
    let mut all_active = use_signal(|| true);
    let mut music_active = use_signal(|| false);
    let mut music_followed_active = use_signal(|| false);
    let mut podcasts_active = use_signal(|| false);
    let mut radio_active = use_signal(|| false);

    let mut activate = move |mut signal: Signal<bool>| {
        let value = *signal.read();
        all_active.set(false);
        music_active.set(false);
        music_followed_active.set(false);
        podcasts_active.set(false);
        radio_active.set(false);
        signal.set(value);
    };

    rsx! {
        div { class: "topbar",
            button {
                class: "settings-icons",
                onclick: move |_| {
                    nav.push(Route::SettingsView {});
                },
                GearIcon { filled: false }
            }
            div { class: "topbar-menu",
                MenuButton {
                    visibility: if music_active() { "hidden" } else { "visible" },
                    display: if music_active() { "none" } else { "flex" },
                    active: all_active,
                    onclick: move |_| {
                        activate(all_active);
                    },
                    "All"
                }
                if *music_active.read() {
                    button {
                        display: "flex",
                        align_items: "center",
                        onclick: move |_| {
                            music_active.set(false);
                            activate(music_active);
                        },
                        CloseIcon { size: 20 }
                    }
                }
                ExpandableButton {
                    active: music_active,
                    inner_active: music_followed_active,
                    onclick: move |_| {
                        activate(music_active);
                    },
                    text: "Music",
                    text_expanded: "Followed",
                }
                MenuButton {
                    visibility: if music_active() { "hidden" } else { "visible" },
                    display: if music_active() { "none" } else { "flex" },
                    active: podcasts_active,
                    onclick: move |_| {
                        activate(podcasts_active);
                    },
                    "Podcasts"
                }
                MenuButton {
                    visibility: if music_active() { "hidden" } else { "visible" },
                    display: if music_active() { "none" } else { "flex" },
                    active: radio_active,
                    onclick: move |_| {
                        activate(radio_active);
                    },
                    "Radio"
                }
            }
        }
    }
}
