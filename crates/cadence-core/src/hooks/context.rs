use crate::player::PlayerCommand;
use crate::state::LoginState;
use dioxus::prelude::*;
use tokio::sync::broadcast;

pub fn init_global_context() {
    debug!("Creating new app context");
    let (command_tx, command_rx) = flume::bounded::<PlayerCommand>(10);
    let (position_tx, _) = tokio::sync::broadcast::channel(10);
    // TODO: flume to
    let _: broadcast::Sender<u64> = use_context_provider(|| position_tx);
    let _ = use_context_provider(|| command_tx);
    let _ = use_context_provider(|| command_rx);
    let _ = use_context_provider(LoginState::default);
}
