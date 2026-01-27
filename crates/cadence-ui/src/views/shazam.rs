use cadence_core::hooks::{use_lidarr_queue_album, use_lidarr_settings, use_shazam_identify};
use dioxus::prelude::*;

#[component]
pub fn ShazamView() -> Element {
    let mut identify = use_shazam_identify();
    let mut is_listening = use_signal(|| false);
    let lidarr_settings = use_lidarr_settings();
    let mut queue_album = use_lidarr_queue_album();
    let mut queue_status = use_signal(|| None::<String>);

    let start_listening = move |_| {
        is_listening.set(true);
        queue_status.set(None);
        identify.call(5); // 5 seconds recording
    };

    let content = match identify.value() {
        Some(Ok(music)) => {
            is_listening.set(false);
            let music_data = music.read();
            let artist = music_data.artist.clone();
            let album = music_data
                .album
                .clone()
                .unwrap_or_else(|| music_data.title.clone());
            let lidarr_configured = lidarr_settings().is_configured();

            let queue_download = move |_| {
                queue_status.set(Some("Queuing...".to_string()));
                queue_album.call(artist.clone(), album.clone());
            };

            // Check queue result
            if let Some(result) = queue_album.value() {
                match result.as_ref() {
                    Ok(msg) => queue_status.set(Some(msg.to_string())),
                    Err(e) => queue_status.set(Some(format!("Error: {e}"))),
                }
            }

            rsx! {
                div { class: "shazam-result",
                    if let Some(cover) = &music_data.cover_art {
                        img { class: "shazam-cover", src: "{cover}" }
                    }
                    div { class: "shazam-info",
                        h2 { class: "shazam-title", "{music_data.title}" }
                        p { class: "shazam-artist", "{music_data.artist}" }
                        if let Some(album) = &music_data.album {
                            p { class: "shazam-album", "{album}" }
                        }
                        if let Some(genre) = &music_data.genre {
                            span { class: "shazam-genre", "{genre}" }
                        }
                    }
                    if lidarr_configured {
                        div { class: "shazam-actions",
                            button {
                                class: "shazam-download-button",
                                onclick: queue_download,
                                disabled: queue_album.pending(),
                                if queue_album.pending() {
                                    "Queuing..."
                                } else {
                                    "Download Album"
                                }
                            }
                            if let Some(status) = queue_status() {
                                p { class: "shazam-queue-status", "{status}" }
                            }
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
        div {
            class: "shazam-view",
            class: if is_listening() { "shazam-view--listening" },
            // Sinewave background overlay when listening
            if is_listening() {
                div { class: "shazam-sinewave-container",
                    div { class: "shazam-sinewave-fill" }
                    svg {
                        class: "shazam-sinewave",
                        view_box: "0 0 200 20",
                        preserve_aspect_ratio: "none",
                        // Two identical wave cycles for seamless looping
                        path {
                            class: "shazam-sinewave-path",
                            d: "M0,10 Q25,0 50,10 T100,10 T150,10 T200,10 L200,20 L0,20 Z",
                        }
                    }
                }
            }
            div { class: "shazam-content",
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
}
