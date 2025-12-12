use dioxus::{CapturedError, prelude::*};
use dioxus_sdk::storage::{get_from_storage, use_storage};

use crate::{
    hooks::effects::initialize_audio_backend,
    model::{Album, PlaylistInfo, SearchResult, Song},
    services::subsonic_client::{AlbumListType, SUBSONIC_CLIENT, SubsonicClient},
    state::{LoginState, SubSonicLogin},
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

pub fn use_login_state() -> LoginState {
    initialize_audio_backend();
    consume_context()
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

async fn fetch_albums(album_type: AlbumListType) -> dioxus::Result<Vec<Album>, CapturedError> {
    let response = SUBSONIC_CLIENT()
        .unwrap()
        .list_album(album_type)
        .await
        .map_err(|err| CapturedError::from_display(format!("{err}")))?;
    Ok(response)
}
