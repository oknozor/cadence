use serde::{Deserialize, Serialize};

// ============================================================================
// Request Models
// ============================================================================

#[derive(Debug, Clone, Serialize)]
pub struct Geolocation {
    pub altitude: f64,
    pub latitude: f64,
    pub longitude: f64,
}

impl Default for Geolocation {
    fn default() -> Self {
        Self {
            altitude: 0.0,
            latitude: 0.0,
            longitude: 0.0,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ShazamSignature {
    pub samplems: i32,
    pub timestamp: i64,
    pub uri: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ShazamRequestBody {
    pub geolocation: Geolocation,
    pub signature: ShazamSignature,
    pub timestamp: i64,
    pub timezone: String,
}

// ============================================================================
// Response Models
// ============================================================================

#[derive(Debug, Clone, Deserialize)]
pub struct ShazamResponse {
    pub track: Option<Track>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Track {
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub genres: Option<Genres>,
    pub images: Option<Images>,
    pub sections: Option<Vec<Section>>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Genres {
    pub primary: Option<String>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Images {
    pub coverart: Option<String>,
    pub coverarthq: Option<String>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Section {
    #[serde(rename = "type")]
    pub section_type: Option<String>,
    pub metadata: Option<Vec<Metadata>>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Metadata {
    pub title: Option<String>,
    pub text: Option<String>,
}

// ============================================================================
// Simplified Result Model
// ============================================================================

/// A simplified representation of an identified song for use in the UI
#[derive(Debug, Clone, PartialEq)]
pub struct ShazamMusic {
    pub title: String,
    pub artist: String,
    pub album: Option<String>,
    pub genre: Option<String>,
    pub cover_art: Option<String>,
    pub label: Option<String>,
    pub released: Option<String>,
}

impl From<Track> for ShazamMusic {
    fn from(track: Track) -> Self {
        // Extract metadata from sections
        let mut album = None;
        let mut label = None;
        let mut released = None;

        if let Some(sections) = &track.sections {
            for section in sections {
                if section.section_type.as_deref() == Some("SONG") {
                    if let Some(metadata) = &section.metadata {
                        for meta in metadata {
                            match meta.title.as_deref() {
                                Some("Album") => album = meta.text.clone(),
                                Some("Label") => label = meta.text.clone(),
                                Some("Released") => released = meta.text.clone(),
                                _ => {}
                            }
                        }
                    }
                }
            }
        }

        ShazamMusic {
            title: track.title.unwrap_or_else(|| "Unknown".to_string()),
            artist: track.subtitle.unwrap_or_else(|| "Unknown".to_string()),
            album,
            genre: track.genres.and_then(|g| g.primary),
            cover_art: track.images.and_then(|i| i.coverarthq.or(i.coverart)),
            label,
            released,
        }
    }
}
