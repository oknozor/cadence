use crate::model::{Geolocation, ShazamMusic, ShazamRequestBody, ShazamResponse, ShazamSignature};
use rand::prelude::IndexedRandom;
use rand::seq::SliceRandom;
use reqwest::Client;
use uuid::Uuid;

const BASE_URL: &str = "https://amp.shazam.com";

/// Error type for Shazam API operations
#[derive(Debug)]
pub enum ShazamError {
    Http(reqwest::Error),
    NoMatch,
    InvalidResponse(String),
}

impl std::fmt::Display for ShazamError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShazamError::Http(e) => write!(f, "HTTP error: {}", e),
            ShazamError::NoMatch => write!(f, "No match found"),
            ShazamError::InvalidResponse(s) => write!(f, "Invalid response: {}", s),
        }
    }
}

impl std::error::Error for ShazamError {}

/// Shazam API client for song identification
pub struct ShazamClient {
    client: Client,
}

impl ShazamClient {
    /// Create a new Shazam client
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    /// Identify a song from an audio signature
    ///
    /// # Arguments
    /// * `signature` - Base64 encoded audio signature from the native library
    /// * `sample_ms` - Duration of the audio sample in milliseconds
    ///
    /// # Returns
    /// * `Ok(ShazamMusic)` - Identified song information
    /// * `Err(ShazamError::NoMatch)` - No song matched the signature
    /// * `Err(ShazamError::Http)` - Network error
    pub async fn identify(
        &self,
        signature: &str,
        sample_ms: i32,
    ) -> Result<ShazamMusic, ShazamError> {
        // Generate random seed for UUIDs
        let seed: String = (0..16).map(|_| rand::random::<char>()).collect();

        // Generate namespace-based UUIDs (SHA1)
        let uuid_dns = Uuid::new_v5(&Uuid::NAMESPACE_DNS, seed.as_bytes()).to_string();
        let uuid_url = Uuid::new_v5(&Uuid::NAMESPACE_URL, seed.as_bytes()).to_string();

        // Select a random user agent
        let user_agent = USER_AGENTS
            .choose(&mut rand::thread_rng())
            .unwrap_or(&USER_AGENTS[0]);

        // Build request body
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;

        let body = ShazamRequestBody {
            signature: ShazamSignature {
                uri: format!("data:audio/vnd.shazam.sig;base64,{}", signature),
                samplems: sample_ms,
                timestamp,
            },
            timestamp,
            timezone: "America/New_York".to_string(),
            geolocation: Geolocation {
                altitude: 0.0,
                latitude: 0.0,
                longitude: 0.0,
            },
        };

        // Build URL with query parameters
        let url = format!(
            "{}/discovery/v5/en/US/android/-/tag/{}/{}",
            BASE_URL, uuid_dns, uuid_url
        );

        let response = self
            .client
            .post(&url)
            .header("User-Agent", *user_agent)
            .header("Content-Language", "en_US")
            .header("Content-Type", "application/json")
            .query(&[
                ("sync", "true"),
                ("webv3", "true"),
                ("sampling", "true"),
                ("connected", ""),
                ("shazamapiversion", "v3"),
                ("sharehub", "true"),
                ("video", "v3"),
            ])
            .json(&body)
            .send()
            .await
            .map_err(ShazamError::Http)?;

        // Get raw response text for debugging
        let raw_response = response.text().await.map_err(ShazamError::Http)?;

        // Log in chunks to avoid Android logcat truncation (limit ~4000 chars)
        tracing::info!(
            "[Shazam] Raw API response length: {} bytes",
            raw_response.len()
        );
        for (i, chunk) in raw_response.as_bytes().chunks(3000).enumerate() {
            if let Ok(s) = std::str::from_utf8(chunk) {
                tracing::info!("[Shazam] Response part {}: {}", i + 1, s);
            }
        }

        let shazam_response: ShazamResponse = serde_json::from_str(&raw_response)
            .map_err(|e| ShazamError::InvalidResponse(format!("JSON parse error: {}", e)))?;

        // Extract track from response
        let track = shazam_response.track.ok_or(ShazamError::NoMatch)?;

        Ok(ShazamMusic::from(track))
    }
}

impl Default for ShazamClient {
    fn default() -> Self {
        Self::new()
    }
}

// User agents from various Android devices to mimic Shazam app requests
const USER_AGENTS: &[&str] = &[
    "Dalvik/2.1.0 (Linux; U; Android 5.0.2; VS980 4G Build/LRX22G)",
    "Dalvik/1.6.0 (Linux; U; Android 4.4.2; SM-T210 Build/KOT49H)",
    "Dalvik/2.1.0 (Linux; U; Android 5.1.1; SM-P905V Build/LMY47X)",
    "Dalvik/1.6.0 (Linux; U; Android 4.4.4; Vodafone Smart Tab 4G Build/KTU84P)",
    "Dalvik/1.6.0 (Linux; U; Android 4.4.4; SM-G360H Build/KTU84P)",
    "Dalvik/2.1.0 (Linux; U; Android 5.0.2; SM-S920L Build/LRX22G)",
    "Dalvik/2.1.0 (Linux; U; Android 5.0; Fire Pro Build/LRX21M)",
    "Dalvik/2.1.0 (Linux; U; Android 5.0; SM-N9005 Build/LRX21V)",
    "Dalvik/2.1.0 (Linux; U; Android 6.0.1; SM-G920F Build/MMB29K)",
    "Dalvik/1.6.0 (Linux; U; Android 4.4.2; SM-G7102 Build/KOT49H)",
    "Dalvik/2.1.0 (Linux; U; Android 5.0; SM-G900F Build/LRX21T)",
    "Dalvik/2.1.0 (Linux; U; Android 6.0.1; SM-G928F Build/MMB29K)",
    "Dalvik/2.1.0 (Linux; U; Android 5.1.1; SM-J500FN Build/LMY48B)",
    "Dalvik/2.1.0 (Linux; U; Android 5.1.1; Coolpad 3320A Build/LMY47V)",
    "Dalvik/1.6.0 (Linux; U; Android 4.4.4; SM-J110F Build/KTU84P)",
    "Dalvik/1.6.0 (Linux; U; Android 4.4.2; SAMSUNG-SGH-I747 Build/KOT49H)",
    "Dalvik/1.6.0 (Linux; U; Android 4.4.2; SAMSUNG-SM-T337A Build/KOT49H)",
    "Dalvik/1.6.0 (Linux; U; Android 4.3; SGH-T999 Build/JSS15J)",
    "Dalvik/2.1.0 (Linux; U; Android 6.0.1; D6603 Build/23.5.A.0.570)",
    "Dalvik/2.1.0 (Linux; U; Android 5.1.1; SM-J700H Build/LMY48B)",
];
