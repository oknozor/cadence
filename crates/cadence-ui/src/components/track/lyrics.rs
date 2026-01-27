use cadence_core::services::genius_client::{LyricsResult, SyncedLyricLine};
use dioxus::CapturedError;
use dioxus::prelude::*;

#[component]
pub fn LyricsCard(
    lyrics: Resource<Result<LyricsResult, CapturedError>>,
    position_ms: Memo<u64>,
) -> Element {
    match lyrics() {
        Some(Ok(result)) => {
            if let Some(synced_lyrics) = &result.synced_lyrics {
                rsx! {
                    SynchronizedLyrics {
                        lines: synced_lyrics.clone(),
                        position_ms,
                        source: result.source.to_string(),
                    }
                }
            } else {
                rsx! {
                    UnsynchronizedLyrics {
                        lyrics: result.lyrics.clone(),
                        source: result.source.to_string(),
                    }
                }
            }
        }
        Some(Err(_)) => {
            rsx! {
                div { class: "lyrics-container lyrics-error",
                    div { class: "lyrics-message", "Lyrics not available" }
                }
            }
        }
        None => {
            rsx! {
                div { class: "lyrics-container lyrics-loading",
                    div { class: "lyrics-message", "Loading lyrics..." }
                }
            }
        }
    }
}

#[component]
fn UnsynchronizedLyrics(lyrics: String, source: String) -> Element {
    rsx! {
        div { class: "lyrics-container",
            div { class: "lyrics-text unsynchronized",
                for (idx , line) in lyrics.lines().enumerate() {
                    p { key: "{idx}", class: "lyric-line", "{line}" }
                }
            }
        }
    }
}

#[component]
fn SynchronizedLyrics(
    lines: Vec<SyncedLyricLine>,
    position_ms: Memo<u64>,
    source: String,
) -> Element {
    let lines_for_memo = lines.clone();

    let current_index = use_memo(move || {
        let pos = position_ms();
        let mut active_idx = 0;
        for (idx, line) in lines_for_memo.iter().enumerate() {
            if pos >= line.time_ms {
                active_idx = idx;
            } else {
                break;
            }
        }
        active_idx
    });

    rsx! {
        div { class: "lyrics-container synchronized",
            div { class: "lyrics-text synchronized",
                for (idx , line) in lines.iter().enumerate() {
                    p {
                        key: "{idx}",
                        id: "lyric-{idx}",
                        class: if idx == current_index() { "lyric-line active" } else { "lyric-line" },
                        "{line.text}"
                    }
                }
            }
        }
    }
}
