use crate::components::{
    BackButton, DotIcon, ItemInfo, LibraryIcon, LyricsCard, NextIcon, PlayIconCircle,
    PlayerProgress, PlusIcon, PreviousIcon, QueueModal, RandomIcon, ShuffleIcon, VerticalScroller,
};
use crate::navigation::navbar::HIDE_PLAYER;
use cadence_core::hooks::use_lyrics;
use cadence_core::state::{ControllerExt, ControllerStoreExt, CONTROLLER};
use dioxus::document::eval;
use dioxus::prelude::*;

#[component]
pub fn NowPlayingView() -> Element {
    use_hook(|| *HIDE_PLAYER.write() = true);
    use_drop(|| *HIDE_PLAYER.write() = false);

    let controller = CONTROLLER.resolve();
    let current = controller.current();

    // Signals for current track info (for lyrics hook)
    let mut current_artist = use_signal(String::new);
    let mut current_title = use_signal(String::new);
    let mut current_duration = use_signal(|| None::<u64>);
    let mut dominant_color = use_signal(|| String::from("rgba(0, 0, 0, 0.3)"));
    let mut queue_open = use_signal(|| false);

    // Update signals when track changes
    use_effect(move || {
        if let Some(track) = current {
            let track_data = track.read();
            if current_artist() != track_data.1.artist || current_title() != track_data.1.title {
                current_artist.set(track_data.1.artist.clone());
                current_title.set(track_data.1.title.clone());
                current_duration.set(track_data.1.duration.map(|d| d as u64));
            }
        } else {
            if !current_artist().is_empty() || !current_title().is_empty() {
                current_artist.set(String::new());
                current_title.set(String::new());
                current_duration.set(None);
            }
        }
    });

    let lyrics = use_lyrics(current_artist, current_title, current_duration);
    let position_ms = use_memo(move || (controller.position().read().as_millis()) as u64);

    let gradient_style = use_memo(move || {
        let color = dominant_color();
        format!("background: {}", color)
    });

    rsx! {
        BackButton {}
        VerticalScroller {
            div { class: "now-playing-view", style: "{gradient_style}",
                if let Some(track) = current {
                    if let Some(src) = track.read().1.cover_art.clone() {
                        AlbumCoverBackground {
                            div { class: "now-playing-header",
                                div { class: "now-playing-header-title",
                                    span { "PLAYING FROM ALBUM" }
                                    h2 { "{track.read().1.album}" }
                                }
                                div { class: "header-end",
                                    button { onclick: move |_| queue_open.set(true),
                                        LibraryIcon { filled: false }
                                    }
                                    DotIcon {}
                                }
                            }

                            img {
                                class: "album-cover-image",
                                id: "now-playing-cover",
                                crossorigin: "anonymous",
                                src,
                                alt: "Album cover",
                                onload: move |_| {
                                    spawn(async move {
                                        let color = extract_vibrant_color().await;
                                        dominant_color.set(color);
                                    });
                                },
                            }

                            div { class: "track-control",
                                div { class: "track-info",
                                    ItemInfo {
                                        primary: track.read().1.title.clone(),
                                        secondary: track.read().1.artist.clone(),
                                    }
                                    PlusIcon { filled: false }
                                }
                                PlayerProgressAndTiming {}
                                PlayerControls {}
                            }
                        }
                    }

                    LyricsCard { lyrics, position_ms }
                }
            }
        }
        QueueModal { open: queue_open }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct AlbumCoverBackgroundProps {
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    pub children: Element,
}

#[component]
pub fn AlbumCoverBackground(props: AlbumCoverBackgroundProps) -> Element {
    rsx! {
        div { class: "album-cover-background", ..props.attributes, {props.children} }
    }
}

#[component]
fn PlayerProgressAndTiming() -> Element {
    let controller = CONTROLLER.resolve();
    let current = controller.current();

    let track_duration = current
        .map(|song| {
            let duration = song.read().1.duration.unwrap_or_default();
            let minutes = duration / 60;
            let seconds = duration % 60;
            format!("{:02}:{:02}", minutes, seconds)
        })
        .unwrap_or("00:00".to_string());

    if let Some(track) = current {
        rsx! {
            PlayerProgress {
                value: controller.position_f64(),
                max: track.read().1.duration.unwrap_or_default() as f64,
            }

            div { class: "timing-progress",
                span { "{controller.position_display()}" }
                span { "{track_duration}" }
            }
        }
    } else {
        rsx! {}
    }
}

#[component]
fn PlayerControls() -> Element {
    let mut controller = CONTROLLER.resolve();
    let is_playing = controller.is_playing()();
    let is_shuffle = controller.shuffle()();
    let is_random = controller.random()();

    let has_next = use_memo(move || {
        let current = *controller.current_idx().read();
        let last = controller.queue_store().read().len();
        current != last
    });

    let has_previous = use_memo(move || {
        let current = *controller.current_idx().read();
        current == 0
    });

    rsx! {
        div { class: "player-controls",
            button {
                class: if !is_shuffle { "control disabled" } else { "control" },
                onclick: move |_| {
                    controller.shuffle().toggle();
                },
                ShuffleIcon { size: 24, filled: is_shuffle }
            }

            div { class: "queue-controls",
                button {
                    class: if !has_previous() { "control disabled" } else { "control" },
                    onclick: move |_| {
                        controller.previous();
                    },
                    PreviousIcon { filled: has_previous() }
                }

                button {
                    class: "control",
                    onclick: move |_| {
                        controller.toggle_play();
                    },
                    PlayIconCircle { size: 48, is_playing }
                }

                button {
                    class: if !has_next() { "control disabled" } else { "control" },
                    onclick: move |_| {
                        controller.next();
                    },
                    NextIcon { size: 24, filled: has_next() }
                }
            }

            button {
                class: if !is_random { "control disabled" } else { "control" },
                onclick: move |_| {
                    controller.random().toggle();
                },
                RandomIcon { filled: is_random }
            }
        }
    }
}

/// Extract the most vibrant color from an image element using eval
async fn extract_vibrant_color() -> String {
    let js_code = r#"
        try {
            const img = document.getElementById('now-playing-cover');
            if (!img || !img.complete) {
                dioxus.send('rgba(0, 0, 0, 0.3)');
                return;
            }

            // Create an off-screen canvas
            const canvas = document.createElement('canvas');
            const sampleSize = 50;
            canvas.width = sampleSize;
            canvas.height = sampleSize;

            const ctx = canvas.getContext('2d');
            if (!ctx) {
                dioxus.send('rgba(0, 0, 0, 0.3)');
                return;
            }

            // Draw the image scaled down
            ctx.drawImage(img, 0, 0, sampleSize, sampleSize);

            // Get image data
            const imageData = ctx.getImageData(0, 0, sampleSize, sampleSize);
            const data = imageData.data;

            // Find the most vibrant (highest saturation) color
            let bestR = 0, bestG = 0, bestB = 0;
            let maxSaturation = 0;

            for (let i = 0; i < data.length; i += 4) {
                const r = data[i];
                const g = data[i + 1];
                const b = data[i + 2];

                // Calculate saturation (difference between max and min RGB values)
                const max = Math.max(r, g, b);
                const min = Math.min(r, g, b);
                const lightness = (max + min) / 2;

                // Skip very dark or very light pixels
                if (lightness < 30 || lightness > 220) continue;

                // Calculate saturation (HSL formula)
                let saturation = 0;
                if (max !== min) {
                    saturation = lightness > 127
                        ? (max - min) / (510 - max - min)
                        : (max - min) / (max + min);
                }

                if (saturation > maxSaturation) {
                    maxSaturation = saturation;
                    bestR = r;
                    bestG = g;
                    bestB = b;
                }
            }

            // If no vibrant color found, fall back to default
            if (maxSaturation === 0) {
                dioxus.send('rgba(0, 0, 0, 0.3)');
                return;
            }

            // Slightly darken the color for a rich gradient effect
            const rDark = Math.floor(bestR * 0.7);
            const gDark = Math.floor(bestG * 0.7);
            const bDark = Math.floor(bestB * 0.7);

            dioxus.send(`rgba(${rDark}, ${gDark}, ${bDark}, 0.85)`);
        } catch (e) {
            console.error('Error extracting color:', e);
            dioxus.send('rgba(0, 0, 0, 0.3)');
        }
    "#;

    let mut eval = eval(js_code);

    eval.recv::<String>()
        .await
        .unwrap_or_else(|_| String::from("rgba(0, 0, 0, 0.3)"))
}
