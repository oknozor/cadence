use crate::components::{BackButton, VerticalScroller};
use cadence_core::hooks::{use_lidarr_settings, use_saved_credentials, use_ticketmaster_settings};
use dioxus::prelude::*;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[component]
pub fn SettingsView() -> Element {
    let credentials = use_saved_credentials();
    let mut lidarr_settings = use_lidarr_settings();
    let mut ticketmaster_settings = use_ticketmaster_settings();

    let (server_url, username) = credentials()
        .map(|c| (c.server_url, c.username))
        .unwrap_or_else(|| ("Not connected".to_string(), "Unknown".to_string()));

    let lidarr_url = use_signal(|| lidarr_settings().url.clone());
    let lidarr_api_key = use_signal(|| lidarr_settings().api_key.clone());

    let ticketmaster_api_key = use_signal(|| ticketmaster_settings().api_key.clone());
    let ticketmaster_cities = use_signal(|| ticketmaster_settings().preferred_cities.join(", "));
    let ticketmaster_radius = use_signal(|| ticketmaster_settings().radius_km.to_string());

    let save_lidarr = move |_| {
        let mut settings = lidarr_settings.write();
        settings.url = lidarr_url().clone();
        settings.api_key = lidarr_api_key().clone();
    };

    let save_ticketmaster = move |_| {
        let mut settings = ticketmaster_settings.write();
        settings.api_key = ticketmaster_api_key().clone();
        settings.preferred_cities = ticketmaster_cities()
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        settings.radius_km = ticketmaster_radius().parse().unwrap_or(50);
    };

    rsx! {
        BackButton {}
        div { class: "settings-view",
            div { class: "settings-header",
                h2 { "Settings" }
            }
            VerticalScroller {
                div { class: "settings-content",
                    SettingsSection { title: "Account",
                        SettingsItem { label: "Server", value: server_url }
                        SettingsItem { label: "Username", value: username }
                    }

                    SettingsSection { title: "Lidarr",
                        SettingsInput {
                            label: "URL",
                            value: lidarr_url,
                            placeholder: "http://localhost:8686",
                        }
                        SettingsInput {
                            label: "API Key",
                            value: lidarr_api_key,
                            placeholder: "Your Lidarr API key",
                        }
                        div { class: "settings-item",
                            button {
                                class: "settings-save-button",
                                onclick: save_lidarr,
                                "Save"
                            }
                        }
                    }

                    SettingsSection { title: "Ticketmaster",
                        SettingsInput {
                            label: "API Key",
                            value: ticketmaster_api_key,
                            placeholder: "Your Ticketmaster API key",
                        }
                        SettingsInput {
                            label: "Cities",
                            value: ticketmaster_cities,
                            placeholder: "Paris, London, Berlin",
                        }
                        SettingsInput {
                            label: "Radius (km)",
                            value: ticketmaster_radius,
                            placeholder: "50",
                        }
                        div { class: "settings-item settings-hint",
                            "Enter cities separated by commas. Radius defines the search area around each city."
                        }
                        div { class: "settings-item",
                            button {
                                class: "settings-save-button",
                                onclick: save_ticketmaster,
                                "Save"
                            }
                        }
                    }

                    SettingsSection { title: "About",
                        SettingsItem { label: "Version", value: VERSION }
                    }
                }
            }
        }
    }
}

#[component]
fn SettingsSection(title: String, children: Element) -> Element {
    rsx! {
        div { class: "settings-section",
            h2 { class: "settings-section-title", "{title}" }
            div { class: "settings-section-content", {children} }
        }
    }
}

#[component]
fn SettingsItem(label: String, value: String) -> Element {
    rsx! {
        div { class: "settings-item",
            span { class: "settings-item-label", "{label}" }
            span { class: "settings-item-value", "{value}" }
        }
    }
}

#[component]
fn SettingsInput(label: String, mut value: Signal<String>, placeholder: String) -> Element {
    rsx! {
        div { class: "settings-item settings-item--input",
            span { class: "settings-item-label", "{label}" }
            input {
                class: "settings-item-input",
                r#type: "text",
                value: "{value}",
                placeholder: "{placeholder}",
                oninput: move |e| {
                    value.set(e.value());
                },
            }
        }
    }
}
