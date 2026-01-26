use cadence_core::hooks::use_shazam_identify;
use dioxus::prelude::*;

#[component]
pub fn ShazamView() -> Element {
    let mut identify = use_shazam_identify();
    let mut is_listening = use_signal(|| false);

    let start_listening = move |_| {
        is_listening.set(true);
        identify.call(5); // 5 seconds recording
    };

    let content = match identify.value() {
        Some(Ok(music)) => {
            is_listening.set(false);
            rsx! {
                div { class: "shazam-result",
                    if let Some(cover) = &music.read().cover_art {
                        img { class: "shazam-cover", src: "{cover}" }
                    }
                    div { class: "shazam-info",
                        h2 { class: "shazam-title", "{music.read().title}" }
                        p { class: "shazam-artist", "{music.read().artist}" }
                        if let Some(album) = &music.read().album {
                            p { class: "shazam-album", "{album}" }
                        }
                        if let Some(genre) = &music.read().genre {
                            span { class: "shazam-genre", "{genre}" }
                        }
                    }
                }
            }
        }
        Some(Err(err)) => {
            is_listening.set(false);
            rsx! {
                div { class: "shazam-error",
                    p { "Could not identify song" }
                    p { class: "shazam-error-detail", "{err}" }
                }
            }
        }
        None if is_listening() => rsx! {
            div { class: "shazam-listening",
                div { class: "shazam-pulse" }
                p { "Listening..." }
            }
        },
        None => rsx! {
            div { class: "shazam-idle",
                p { "Tap to identify a song" }
            }
        },
    };

    rsx! {
        div { class: "shazam-view",
            {content}
            button {
                class: "shazam-button",
                class: if is_listening() { "listening" },
                disabled: is_listening(),
                onclick: start_listening,
                div { class: "shazam-icon" }
            }
        }
    }
}

