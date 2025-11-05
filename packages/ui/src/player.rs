//! Music player component

use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Track {
    pub id: String,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub duration: Option<u32>,
}

#[component]
pub fn Player(
    current_track: Option<Track>,
    on_play: EventHandler<()>,
    on_pause: EventHandler<()>,
    on_next: EventHandler<()>,
    on_previous: EventHandler<()>,
) -> Element {
    let mut is_playing = use_signal(|| false);

    rsx! {
        div {
            class: "player-container",

            if let Some(track) = current_track {
                div {
                    class: "track-info",
                    h3 { "{track.title}" }
                    p { "{track.artist} - {track.album}" }
                }

                div {
                    class: "player-controls",
                    button {
                        onclick: move |_| on_previous.call(()),
                        "⏮ Previous"
                    }

                    button {
                        onclick: move |_| {
                            if *is_playing.read() {
                                on_pause.call(());
                                is_playing.set(false);
                            } else {
                                on_play.call(());
                                is_playing.set(true);
                            }
                        },
                        if *is_playing.read() {
                            "⏸ Pause"
                        } else {
                            "▶ Play"
                        }
                    }

                    button {
                        onclick: move |_| on_next.call(()),
                        "Next ⏭"
                    }
                }
            } else {
                div {
                    class: "no-track",
                    p { "No track loaded" }
                }
            }
        }
    }
}
