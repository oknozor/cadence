use reqwest::Client;
use scraper::{Html, Selector};
use serde::Deserialize;

const GENIUS_SEARCH_URL: &str = "https://genius.com/api/search/song";
const LRCLIB_SEARCH_URL: &str = "https://lrclib.net/api/search";
const LRCLIB_GET_URL: &str = "https://lrclib.net/api/get";

#[derive(Clone)]
pub struct LyricsClient {
    client: Client,
}

#[derive(Debug)]
pub enum LyricsError {
    Http(reqwest::Error),
    NotFound(String),
    ParseError(String),
}

impl std::fmt::Display for LyricsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LyricsError::Http(e) => write!(f, "HTTP error: {}", e),
            LyricsError::NotFound(msg) => write!(f, "Not found: {}", msg),
            LyricsError::ParseError(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

impl std::error::Error for LyricsError {}

impl From<reqwest::Error> for LyricsError {
    fn from(e: reqwest::Error) -> Self {
        LyricsError::Http(e)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LyricSource {
    LrcLib,
    Genius,
}

impl std::fmt::Display for LyricSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LyricSource::LrcLib => write!(f, "lrclib.net"),
            LyricSource::Genius => write!(f, "Genius"),
        }
    }
}

/// A single line of synchronized lyrics with a timestamp in milliseconds
#[derive(Debug, Clone, PartialEq)]
pub struct SyncedLyricLine {
    pub time_ms: u64,
    pub text: String,
}

/// Lyrics result that can be either plain text or synchronized
#[derive(Debug, Clone)]
pub struct LyricsResult {
    pub title: String,
    pub artist: String,
    pub source: LyricSource,
    /// Plain text lyrics (always available)
    pub lyrics: String,
    /// Synchronized lyrics with timestamps (if available)
    pub synced_lyrics: Option<Vec<SyncedLyricLine>>,
}

impl LyricsResult {
    pub fn is_synced(&self) -> bool {
        self.synced_lyrics.is_some()
    }
}

// Genius API Response types
#[derive(Debug, Deserialize)]
struct GeniusSearchResponse {
    response: GeniusResponse,
}

#[derive(Debug, Deserialize)]
struct GeniusResponse {
    sections: Option<Vec<GeniusSection>>,
}

#[derive(Debug, Deserialize)]
struct GeniusSection {
    hits: Vec<GeniusHit>,
}

#[derive(Debug, Deserialize)]
struct GeniusHit {
    result: GeniusSongResult,
}

#[derive(Debug, Deserialize)]
struct GeniusSongResult {
    title: String,
    artist_names: String,
    url: String,
}

// LrcLib API Response types
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LrcLibSearchResult {
    name: String,
    artist_name: String,
    #[serde(default)]
    plain_lyrics: Option<String>,
    #[serde(default)]
    synced_lyrics: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LrcLibTrackResponse {
    name: String,
    artist_name: String,
    #[serde(default)]
    plain_lyrics: Option<String>,
    #[serde(default)]
    synced_lyrics: Option<String>,
}

impl LyricsClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    /// Get lyrics for a song, trying LrcLib first (for synced lyrics), then Genius
    pub async fn get_lyrics(
        &self,
        artist: &str,
        title: &str,
        duration_secs: Option<u64>,
    ) -> Result<LyricsResult, LyricsError> {
        // Try LrcLib first (provides synchronized lyrics)
        match self.get_lrclib_lyrics(artist, title, duration_secs).await {
            Ok(result) => {
                tracing::info!(
                    "[Lyrics] Found lyrics from LrcLib (synced: {})",
                    result.is_synced()
                );
                return Ok(result);
            }
            Err(e) => {
                tracing::info!("[Lyrics] LrcLib failed: {}, trying Genius", e);
            }
        }

        // Fall back to Genius
        self.get_genius_lyrics(artist, title).await
    }

    /// Get lyrics from LrcLib (supports synchronized lyrics)
    async fn get_lrclib_lyrics(
        &self,
        artist: &str,
        title: &str,
        duration_secs: Option<u64>,
    ) -> Result<LyricsResult, LyricsError> {
        tracing::info!("[LrcLib] Searching for: {} - {}", artist, title);

        // Try direct get first if we have duration
        if let Some(duration) = duration_secs {
            let response = self
                .client
                .get(LRCLIB_GET_URL)
                .header(
                    "User-Agent",
                    "Cadence/1.0 (https://github.com/oknozor/cadence)",
                )
                .query(&[
                    ("track_name", title),
                    ("artist_name", artist),
                    ("duration", &duration.to_string()),
                ])
                .send()
                .await?;

            if response.status().is_success() {
                let track: LrcLibTrackResponse = response.json().await?;
                return self.parse_lrclib_result(
                    track.name,
                    track.artist_name,
                    track.plain_lyrics,
                    track.synced_lyrics,
                );
            }
        }

        // Fall back to search
        let results: Vec<LrcLibSearchResult> = self
            .client
            .get(LRCLIB_SEARCH_URL)
            .header(
                "User-Agent",
                "Cadence/1.0 (https://github.com/oknozor/cadence)",
            )
            .query(&[("q", &format!("{} {}", artist, title))])
            .send()
            .await?
            .json()
            .await?;

        let best_match = results
            .into_iter()
            .find(|r| r.plain_lyrics.is_some() || r.synced_lyrics.is_some())
            .ok_or_else(|| LyricsError::NotFound("No lyrics found on LrcLib".to_string()))?;

        self.parse_lrclib_result(
            best_match.name,
            best_match.artist_name,
            best_match.plain_lyrics,
            best_match.synced_lyrics,
        )
    }

    fn parse_lrclib_result(
        &self,
        title: String,
        artist: String,
        plain_lyrics: Option<String>,
        synced_lyrics: Option<String>,
    ) -> Result<LyricsResult, LyricsError> {
        let synced = synced_lyrics.as_ref().map(|s| self.parse_lrc(s));
        let lyrics = synced_lyrics
            .clone()
            .or(plain_lyrics)
            .ok_or_else(|| LyricsError::NotFound("No lyrics content".to_string()))?;

        // If we have synced lyrics, extract plain text from them
        let plain_text = if let Some(ref synced_lines) = synced {
            synced_lines
                .iter()
                .map(|l| l.text.as_str())
                .collect::<Vec<_>>()
                .join("\n")
        } else {
            lyrics
        };

        Ok(LyricsResult {
            title,
            artist,
            source: LyricSource::LrcLib,
            lyrics: plain_text,
            synced_lyrics: synced,
        })
    }

    /// Parse LRC format timestamps into SyncedLyricLine
    fn parse_lrc(&self, lrc: &str) -> Vec<SyncedLyricLine> {
        let mut lines = Vec::new();

        for line in lrc.lines() {
            // LRC format: [mm:ss.xx] text
            if let Some(rest) = line.strip_prefix('[') {
                if let Some(bracket_end) = rest.find(']') {
                    let timestamp = &rest[..bracket_end];
                    let text = rest[bracket_end + 1..].trim().to_string();

                    if let Some(time_ms) = self.parse_lrc_timestamp(timestamp) {
                        lines.push(SyncedLyricLine { time_ms, text });
                    }
                }
            }
        }

        lines
    }

    /// Parse LRC timestamp (mm:ss.xx) to milliseconds
    fn parse_lrc_timestamp(&self, timestamp: &str) -> Option<u64> {
        let parts: Vec<&str> = timestamp.split(':').collect();
        if parts.len() != 2 {
            return None;
        }

        let minutes: u64 = parts[0].parse().ok()?;
        let seconds_parts: Vec<&str> = parts[1].split('.').collect();
        let seconds: u64 = seconds_parts[0].parse().ok()?;
        let centiseconds: u64 = seconds_parts
            .get(1)
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);

        Some(minutes * 60 * 1000 + seconds * 1000 + centiseconds * 10)
    }

    /// Get lyrics from Genius (plain text only)
    async fn get_genius_lyrics(
        &self,
        artist: &str,
        title: &str,
    ) -> Result<LyricsResult, LyricsError> {
        let search_query = format!("{} {}", artist, title);
        tracing::info!("[Genius] Searching for: {}", search_query);

        let response = self
            .client
            .get(GENIUS_SEARCH_URL)
            .query(&[("per_page", "1"), ("q", &search_query)])
            .send()
            .await?
            .json::<GeniusSearchResponse>()
            .await?;

        let hit = response
            .response
            .sections
            .and_then(|s| s.into_iter().next())
            .and_then(|s| s.hits.into_iter().next())
            .ok_or_else(|| LyricsError::NotFound(format!("No results for '{}'", search_query)))?;

        let song_url = &hit.result.url;
        tracing::info!("[Genius] Found song URL: {}", song_url);

        let lyrics = self.fetch_genius_lyrics_from_page(song_url).await?;

        Ok(LyricsResult {
            title: hit.result.title,
            artist: hit.result.artist_names,
            source: LyricSource::Genius,
            lyrics,
            synced_lyrics: None,
        })
    }

    async fn fetch_genius_lyrics_from_page(&self, url: &str) -> Result<String, LyricsError> {
        let html = self.client.get(url).send().await?.text().await?;

        // Replace <br/> with newlines before parsing
        let html = html.replace("<br/>", "\n");
        let document = Html::parse_document(&html);

        // Try the old lyrics div first
        let lyrics_selector = Selector::parse("div.lyrics")
            .map_err(|e| LyricsError::ParseError(format!("Invalid selector: {:?}", e)))?;

        if let Some(lyrics_div) = document.select(&lyrics_selector).next() {
            let text = lyrics_div.text().collect::<Vec<_>>().join("");
            return Ok(text.trim().to_string());
        }

        // Try the new lyrics container
        let container_selector = Selector::parse(r#"div[data-lyrics-container="true"]"#)
            .map_err(|e| LyricsError::ParseError(format!("Invalid selector: {:?}", e)))?;

        let lyrics: String = document
            .select(&container_selector)
            .map(|el| el.text().collect::<Vec<_>>().join(""))
            .collect::<Vec<_>>()
            .join("\n");

        if lyrics.is_empty() {
            return Err(LyricsError::NotFound(
                "Lyrics not found on page".to_string(),
            ));
        }

        Ok(lyrics.trim().to_string())
    }
}

impl Default for LyricsClient {
    fn default() -> Self {
        Self::new()
    }
}
