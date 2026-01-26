use dioxus::{CapturedError, prelude::*};
use dioxus_sdk::storage::{get_from_storage, use_storage};

use crate::{
    model::{Album, PlaylistInfo, RadioStation, SearchResult, ShazamMusic, Song},
    services::{
        subsonic_client::{AlbumListType, SUBSONIC_CLIENT, Star, SubsonicClient},
        ticketmaster_client::Concert,
    },
    state::{LidarrSettings, SubSonicLogin, TicketmasterSettings},
};

#[cfg(target_os = "android")]
use crate::services::{
    shazam_bridge::{self, ShazamBridgeMessage},
    shazam_client::ShazamClient,
};

mod context;
use crate::model::Artist;
pub use context::init_global_context;

pub mod effects;

pub fn use_search_results() -> Action<(String,), Vec<SearchResult>> {
    use_action(move |input: String| async move {
        if input.is_empty() {
            return Ok(vec![]);
        }

        let client = SUBSONIC_CLIENT.cloned().unwrap();
        client
            .search(&input)
            .await
            .map_err(CapturedError::from_display)
    })
}

pub fn use_subsonic_client() -> Option<SubsonicClient> {
    consume_context()
}

pub fn use_saved_credentials() -> Signal<Option<SubSonicLogin>> {
    #[cfg(feature = "mobile")]
    use cadence_storage_android::LocalStorage;

    #[cfg(not(feature = "mobile"))]
    use dioxus_sdk::storage::LocalStorage;

    let saved = get_from_storage::<LocalStorage, Option<SubSonicLogin>>(
        "subsonic_credentials".to_string(),
        || None,
    );

    use_storage::<LocalStorage, _>("subsonic_credentials".to_string(), || saved)
}

pub fn use_lidarr_settings() -> Signal<LidarrSettings> {
    #[cfg(feature = "mobile")]
    use cadence_storage_android::LocalStorage;

    #[cfg(not(feature = "mobile"))]
    use dioxus_sdk::storage::LocalStorage;

    let saved = get_from_storage::<LocalStorage, LidarrSettings>(
        "lidarr_settings".to_string(),
        LidarrSettings::default,
    );

    use_storage::<LocalStorage, _>("lidarr_settings".to_string(), || saved)
}

pub fn use_album(id: Signal<String>) -> Resource<Album> {
    use_resource(move || {
        let id = id;
        async move {
            SUBSONIC_CLIENT()
                .clone()
                .unwrap()
                .get_album(id.read().as_ref())
                .await
                .unwrap()
        }
    })
}

pub fn use_artist(id: Signal<String>) -> Resource<Artist> {
    use_resource(move || async move {
        SUBSONIC_CLIENT()
            .clone()
            .unwrap()
            .get_artist(id.read().as_ref())
            .await
            .unwrap()
    })
}

pub fn use_recently_released() -> Resource<Result<Vec<Album>, CapturedError>> {
    use_resource(|| fetch_albums(AlbumListType::RecentlyReleased))
}

pub fn use_recently_played() -> Resource<Result<Vec<Album>, CapturedError>> {
    use_resource(|| fetch_albums(AlbumListType::RecentlyPlayed))
}

pub fn use_all_playlist() -> Resource<Result<Vec<PlaylistInfo>, CapturedError>> {
    use_resource(move || async move {
        SUBSONIC_CLIENT()
            .clone()
            .unwrap()
            .get_public_playlist()
            .await
            .map_err(|err| CapturedError::from_display(format!("{err}")))
    })
}

pub fn use_random_songs(limit: i64) -> Resource<Result<Vec<Song>, CapturedError>> {
    use_resource(move || async move {
        SUBSONIC_CLIENT()
            .clone()
            .unwrap()
            .get_random_songs(limit)
            .await
            .map_err(|err| CapturedError::from_display(format!("{err}")))
    })
}

pub fn use_star_song() -> Action<(String,), ()> {
    use_action(move |id| async move {
        SUBSONIC_CLIENT()
            .unwrap()
            .star(Star::Song(id))
            .await
            .map_err(|err| CapturedError::from_display(format!("{err}")))
    })
}

pub fn use_star_album() -> Action<(String,), ()> {
    use_action(move |id| async move {
        SUBSONIC_CLIENT()
            .unwrap()
            .star(Star::Album(id))
            .await
            .map_err(|err| CapturedError::from_display(format!("{err}")))
    })
}

pub fn use_star_artist() -> Action<(String,), ()> {
    use_action(move |id| async move {
        SUBSONIC_CLIENT()
            .unwrap()
            .star(Star::Album(id))
            .await
            .map_err(|err| CapturedError::from_display(format!("{err}")))
    })
}

pub fn use_internet_radio_stations() -> Resource<Result<Vec<RadioStation>, CapturedError>> {
    use_resource(move || async move {
        SUBSONIC_CLIENT()
            .clone()
            .unwrap()
            .get_internet_radio_stations()
            .await
            .map_err(|err| CapturedError::from_display(format!("{err}")))
    })
}

async fn fetch_albums(album_type: AlbumListType) -> dioxus::Result<Vec<Album>, CapturedError> {
    let response = SUBSONIC_CLIENT()
        .unwrap()
        .list_album(album_type)
        .await
        .map_err(|err| CapturedError::from_display(format!("{err}")))?;
    Ok(response)
}

/// State for Shazam identification process
#[derive(Debug, Clone, PartialEq)]
pub enum ShazamState {
    Idle,
    Recording,
    Identifying,
    Success(ShazamMusic),
    Error(String),
    NoMatch,
}

#[cfg(target_os = "android")]
pub fn use_shazam_identify() -> Action<(i32,), ShazamMusic> {
    use_action(move |duration_seconds: i32| async move {
        tracing::info!(
            "[Shazam Hook] use_shazam_identify action called with duration={}",
            duration_seconds
        );

        // Initialize channel
        let (tx, rx) = flume::unbounded();
        shazam_bridge::init(tx);
        tracing::info!("[Shazam Hook] Bridge initialized");

        // Start recording
        tracing::info!("[Shazam Hook] Calling start_identification...");
        shazam_bridge::start_identification(duration_seconds).map_err(|e| {
            tracing::error!("[Shazam Hook] start_identification failed: {e}");
            CapturedError::from_display(format!("Failed to start: {e}"))
        })?;
        tracing::info!("[Shazam Hook] start_identification returned OK, waiting for messages...");

        // Wait for signature
        loop {
            tracing::info!("[Shazam Hook] Waiting for message from bridge...");
            match rx.recv_async().await {
                Ok(ShazamBridgeMessage::SignatureReady {
                    signature,
                    sample_ms,
                }) => {
                    tracing::info!(
                        "[Shazam Hook] Received SignatureReady ({} ms, {} bytes)",
                        sample_ms,
                        signature.len()
                    );
                    let client = ShazamClient::new();
                    tracing::info!("[Shazam Hook] Calling Shazam API...");
                    return client.identify(&signature, sample_ms).await.map_err(|e| {
                        tracing::error!("[Shazam Hook] Shazam API error: {e:?}");
                        CapturedError::from_display(format!("{e:?}"))
                    });
                }
                Ok(ShazamBridgeMessage::Error(e)) => {
                    tracing::error!("[Shazam Hook] Received Error: {e}");
                    return Err(CapturedError::from_display(e));
                }
                Ok(msg) => {
                    tracing::info!("[Shazam Hook] Received message: {:?}", msg);
                    continue;
                }
                Err(e) => {
                    tracing::error!("[Shazam Hook] Channel receive error: {e}");
                    return Err(CapturedError::from_display(format!("Channel error: {e}")));
                }
            }
        }
    })
}

pub fn use_lidarr_queue_album() -> Action<(String, String), String> {
    use crate::services::lidarr_client::LidarrClient;

    use_action(move |artist: String, album: String| async move {
        // Get lidarr settings from storage
        #[cfg(feature = "mobile")]
        use cadence_storage_android::LocalStorage;
        #[cfg(not(feature = "mobile"))]
        use dioxus_sdk::storage::LocalStorage;

        let settings = get_from_storage::<LocalStorage, LidarrSettings>(
            "lidarr_settings".to_string(),
            LidarrSettings::default,
        );

        if !settings.is_configured() {
            return Err(CapturedError::from_display(
                "Lidarr not configured. Please set URL and API key in Settings.",
            ));
        }

        let client = LidarrClient::new(&settings);

        // Search for the album
        tracing::info!("[Lidarr Hook] Searching for album: {} - {}", artist, album);
        let albums = client
            .search_album(&artist, &album)
            .await
            .map_err(|e| CapturedError::from_display(format!("{e}")))?;

        let target_album = albums.into_iter().next().ok_or_else(|| {
            CapturedError::from_display(format!("Album not found: {} - {}", artist, album))
        })?;

        tracing::info!("[Lidarr Hook] Found album: {:?}", target_album.title);

        // Add album to Lidarr (this also triggers a search)
        let added = client
            .add_album(&target_album)
            .await
            .map_err(|e| CapturedError::from_display(format!("{e}")))?;

        let album_title = added.title.unwrap_or_else(|| "Unknown".to_string());
        tracing::info!("[Lidarr Hook] Album queued: {}", album_title);

        Ok(format!("Queued: {}", album_title))
    })
}

pub fn use_ticketmaster_settings() -> Signal<TicketmasterSettings> {
    #[cfg(feature = "mobile")]
    use cadence_storage_android::LocalStorage;

    #[cfg(not(feature = "mobile"))]
    use dioxus_sdk::storage::LocalStorage;

    let saved = get_from_storage::<LocalStorage, TicketmasterSettings>(
        "ticketmaster_settings".to_string(),
        TicketmasterSettings::default,
    );

    use_storage::<LocalStorage, _>("ticketmaster_settings".to_string(), || saved)
}

pub fn use_artist_concerts(
    artist_name: Signal<String>,
) -> Resource<Result<Vec<Concert>, CapturedError>> {
    use crate::services::ticketmaster_client::TicketmasterClient;

    tracing::info!("[Ticketmaster] use_artist_concerts hook called");

    use_resource(move || {
        let artist = artist_name();
        tracing::info!("[Ticketmaster] Resource triggered for artist: '{}'", artist);
        async move {
            tracing::info!("[Ticketmaster] Async block started for artist: '{}'", artist);

            if artist.is_empty() {
                tracing::info!("[Ticketmaster] Artist name is empty, returning empty list");
                return Ok(vec![]);
            }

            // Get settings from storage
            #[cfg(feature = "mobile")]
            use cadence_storage_android::LocalStorage;
            #[cfg(not(feature = "mobile"))]
            use dioxus_sdk::storage::LocalStorage;

            let settings = get_from_storage::<LocalStorage, TicketmasterSettings>(
                "ticketmaster_settings".to_string(),
                TicketmasterSettings::default,
            );

            tracing::info!(
                "[Ticketmaster] Settings loaded - api_key: '{}', cities: {:?}, radius: {}km, configured: {}",
                settings.api_key,
                settings.preferred_cities,
                settings.radius_km,
                settings.is_configured()
            );

            if !settings.is_configured() {
                tracing::info!("[Ticketmaster] Not configured, skipping concert search");
                return Ok(vec![]);
            }

            let client = TicketmasterClient::new(&settings.api_key);
            let mut all_concerts = Vec::new();
            let radius = if settings.radius_km > 0 {
                Some(settings.radius_km)
            } else {
                None
            };

            // Search for events in each preferred city
            for city in &settings.preferred_cities {
                tracing::info!("[Ticketmaster] Searching for '{}' in '{}' (radius: {:?} km)", artist, city, radius);
                match client.search_events(&artist, city, radius).await {
                    Ok(concerts) => {
                        tracing::info!("[Ticketmaster] Found {} concerts in {}", concerts.len(), city);
                        all_concerts.extend(concerts);
                    }
                    Err(e) => {
                        tracing::warn!("[Ticketmaster] Error searching {}: {}", city, e);
                    }
                }
            }

            // Sort by date
            all_concerts.sort_by(|a, b| a.date.cmp(&b.date));

            // Remove duplicates (same event might appear in multiple city searches)
            all_concerts.dedup_by(|a, b| a.url == b.url);

            tracing::info!("[Ticketmaster] Total concerts found: {}", all_concerts.len());
            Ok(all_concerts)
        }
    })
}
