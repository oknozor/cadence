use crate::components::{CloseIcon, ExpandableButton, GearIcon, MenuButton};
use crate::views::Route;
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TopBarFilter {
    All,
    Music,
    MusicFollowed,
    Podcasts,
    Radio,
}

#[derive(Props, Clone, PartialEq)]
pub struct TopBarProps {
    active_filter: Signal<TopBarFilter>,
}

#[component]
pub fn TopBar(props: TopBarProps) -> Element {
    let nav = navigator();
    let mut active_filter = props.active_filter;

    let mut all_active = use_signal(|| active_filter() == TopBarFilter::All);
    let mut music_active = use_signal(|| active_filter() == TopBarFilter::Music);
    let mut music_followed_active = use_signal(|| active_filter() == TopBarFilter::MusicFollowed);
    let mut podcasts_active = use_signal(|| active_filter() == TopBarFilter::Podcasts);
    let mut radio_active = use_signal(|| active_filter() == TopBarFilter::Radio);

    use_effect(move || {
        all_active.set(active_filter() == TopBarFilter::All);
        music_active.set(active_filter() == TopBarFilter::Music);
        music_followed_active.set(active_filter() == TopBarFilter::MusicFollowed);
        podcasts_active.set(active_filter() == TopBarFilter::Podcasts);
        radio_active.set(active_filter() == TopBarFilter::Radio);
    });

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
                    visibility: if active_filter() == TopBarFilter::Music { "hidden" } else { "visible" },
                    display: if active_filter() == TopBarFilter::Music { "none" } else { "flex" },
                    active: all_active,
                    onclick: move |_| {
                        active_filter.set(TopBarFilter::All);
                    },
                    "All"
                }
                if active_filter() == TopBarFilter::Music {
                    button {
                        display: "flex",
                        align_items: "center",
                        onclick: move |_| {
                            active_filter.set(TopBarFilter::All);
                        },
                        CloseIcon { size: 20 }
                    }
                }
                ExpandableButton {
                    active: music_active,
                    inner_active: music_followed_active,
                    onclick: move |_| {
                        active_filter.set(TopBarFilter::Music);
                    },
                    text: "Music",
                    text_expanded: "Followed",
                }
                MenuButton {
                    visibility: if active_filter() == TopBarFilter::Music { "hidden" } else { "visible" },
                    display: if active_filter() == TopBarFilter::Music { "none" } else { "flex" },
                    active: podcasts_active,
                    onclick: move |_| {
                        active_filter.set(TopBarFilter::Podcasts);
                    },
                    "Podcasts"
                }
                MenuButton {
                    visibility: if active_filter() == TopBarFilter::Music { "hidden" } else { "visible" },
                    display: if active_filter() == TopBarFilter::Music { "none" } else { "flex" },
                    active: radio_active,
                    onclick: move |_| {
                        active_filter.set(TopBarFilter::Radio);
                    },
                    "Radio"
                }
            }
        }
    }
}
