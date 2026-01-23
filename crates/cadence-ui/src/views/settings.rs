use crate::components::{BackButton, VerticalScroller};
use cadence_core::hooks::use_saved_credentials;
use dioxus::prelude::*;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[component]
pub fn SettingsView() -> Element {
    let credentials = use_saved_credentials();

    let (server_url, username) = credentials()
        .map(|c| (c.server_url, c.username))
        .unwrap_or_else(|| ("Not connected".to_string(), "Unknown".to_string()));

    rsx! {
        BackButton {}
        div { class: "settings-view",
            div { class: "settings-header",
                h2 { "Settings" }
            }
            VerticalScroller {
                div { class: "settings-content",
                    SettingsSection {
                        title: "Account",
                        SettingsItem {
                            label: "Server",
                            value: server_url,
                        }
                        SettingsItem {
                            label: "Username",
                            value: username,
                        }
                    }

                    SettingsSection {
                        title: "About",
                        SettingsItem {
                            label: "Version",
                            value: VERSION,
                        }
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
