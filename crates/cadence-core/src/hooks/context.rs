use cadence_player::PlayerCommand;
use dioxus::prelude::*;
use std::sync::Arc;
use tokio::sync::{Mutex, broadcast};

use crate::state::{LoginState, PlayerState, QueueState};

pub fn init_global_context() {
    let (command_tx, command_rx) = tokio::sync::mpsc::channel::<PlayerCommand>(10);
    let (position_tx, _) = tokio::sync::broadcast::channel(10);
    let _: broadcast::Sender<u64> = use_context_provider(|| position_tx);
    let _ = use_context_provider(|| Arc::new(Mutex::new(command_rx)));
    let _ = use_context_provider(|| command_tx);
    let _ = use_context_provider(QueueState::default);
    let _ = use_context_provider(PlayerState::default);
    let _ = use_context_provider(LoginState::default);
}
