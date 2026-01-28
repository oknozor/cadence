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

const VISIBLE_LINES: usize = 5;

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

    let visible_data = use_memo(move || {
        let idx = current_index();
        let total = lines.len();
        let half = VISIBLE_LINES / 2;

        let mut start = idx.saturating_sub(half);
        let end = (start + VISIBLE_LINES).min(total);
        start = end.saturating_sub(VISIBLE_LINES);

        let active_in_visible = idx - start;

        let visible = lines
            .iter()
            .skip(start)
            .take(VISIBLE_LINES.min(total))
            .cloned()
            .collect::<Vec<_>>();

        (visible, active_in_visible)
    });

    rsx! {
        div { class: "lyrics-preview",
            div { class: "lyrics-preview-header",
                span { "Lyrics preview" }
            }
            div { class: "lyrics-preview-lines",
                for (idx , line) in visible_data().0.iter().enumerate() {
                    p {
                        key: "{idx}",
                        class: if idx == visible_data().1 { "lyric-line active" } else { "lyric-line" },
                        "{line.text}"
                    }
                }
            }
            button { class: "lyrics-show-button", "Show lyrics" }
        }
    }
}
