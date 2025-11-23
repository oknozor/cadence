use cadence_player::PlayerCommand;
use dioxus::prelude::*;
use tokio::sync::broadcast;

use crate::{
    model::Song,
    state::{PlayerState, QueueState},
};

pub fn use_playback_position() -> Signal<Option<f64>> {
    let position_tx: broadcast::Sender<u64> = use_context();
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

pub fn use_command_sender() -> tokio::sync::mpsc::Sender<PlayerCommand> {
    consume_context()
}

pub fn use_player_state() -> PlayerState {
    consume_context()
}

pub fn use_queue_state() -> QueueState {
    consume_context()
}
