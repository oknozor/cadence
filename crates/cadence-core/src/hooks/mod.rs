use crate::player::PlayerCommand;
use dioxus::{CapturedError, prelude::*};
use dioxus_sdk::storage::{get_from_storage, use_storage};
use tokio::sync::broadcast;

use crate::{
    hooks::effects::use_on_login_effect,
    model::{Album, SearchResult},
    services::subsonic_client::{AlbumListType, SUBSONIC_CLIENT, SubsonicClient},
    state::{LoginState, SubSonicLogin},
};

mod context;
pub use context::init_global_context;

pub mod effects;

pub fn use_playback_position() -> Signal<Option<f64>> {
    let position_tx: broadcast::Sender<u64> = use_playback_position_sender();
    let mut position = use_signal(|| None::<f64>);

    use_effect(move || {
        let mut tx = position_tx.subscribe();
        spawn(async move {
            while let Ok(new_progress) = tx.recv().await {
                position.set(Some(new_progress as f64));
            }
        });
    });

    position
}

pub fn use_playback_position_sender() -> broadcast::Sender<u64> {
    consume_context()
}

pub fn use_search_results() -> Action<(String,), Vec<SearchResult>> {
    use_action(move |input: String| async move {
        if input.is_empty() {
            return Ok(vec![]);
        }

        let client = SUBSONIC_CLIENT.cloned().unwrap();
        client
            .search(&input)
            .await
            .map_err(|err| CapturedError::from_display(err))
    })
}

pub fn use_login_state() -> LoginState {
    use_on_login_effect();
    consume_context()
}

pub fn use_command_sender() -> flume::Sender<PlayerCommand> {
    consume_context()
}

pub fn use_command_receiver() -> flume::Receiver<PlayerCommand> {
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

pub fn use_recently_released() -> Resource<Result<Vec<Album>, CapturedError>> {
    use_resource(|| fetch_albums(AlbumListType::RecentlyReleased))
}

pub fn use_recently_played() -> Resource<Result<Vec<Album>, CapturedError>> {
    use_resource(|| fetch_albums(AlbumListType::RecentlyPlayed))
}

async fn fetch_albums(album_type: AlbumListType) -> dioxus::Result<Vec<Album>, CapturedError> {
    let response = SUBSONIC_CLIENT()
        .unwrap()
        .list_album(album_type)
        .await
        .map_err(|err| CapturedError::from_display(format!("{err}")))?;
    Ok(response)
}
