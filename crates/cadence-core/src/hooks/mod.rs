use std::sync::Arc;

use cadence_player::PlayerCommand;
use dioxus::prelude::*;
use dioxus_sdk::storage::{get_from_storage, use_storage};
use tokio::sync::{Mutex, broadcast, mpsc};

use crate::{
    hooks::effects::use_on_login_effect,
    model::Song,
    services::subsonic_client::SubsonicClient,
    state::{LoginState, PlayerState, QueueState, SubSonicLogin},
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

pub fn use_current_track() -> Memo<Option<Song>> {
    let queue = consume_context::<QueueState>();
    use_memo(move || {
        queue
            .current_track_idx()
            .map(|idx| queue.songs().get(idx).cloned())
            .flatten()
    })
}

pub fn use_playback_position_sender() -> broadcast::Sender<u64> {
    consume_context()
}

pub fn use_login_state() -> LoginState {
    use_on_login_effect();
    consume_context()
}

pub fn use_command_sender() -> mpsc::Sender<PlayerCommand> {
    consume_context()
}

pub fn use_command_receiver() -> Arc<Mutex<mpsc::Receiver<PlayerCommand>>> {
    consume_context()
}

pub fn use_player_state() -> PlayerState {
    consume_context()
}

pub fn use_queue_state() -> QueueState {
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
