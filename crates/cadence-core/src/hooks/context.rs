use crate::player::{AudioBackendStateUpdate, PlayerCommand};
use crate::state::{HostNotificationCommand, LoginState};
use dioxus::prelude::*;

pub fn init_global_context() {
    debug!("Creating new app context");
    let (command_tx, command_rx) = flume::bounded::<PlayerCommand>(150);
    let (notification_tx, notification_rx) = flume::bounded::<HostNotificationCommand>(10);
    let (audio_backend_state_tx, audio_backend_state_rx) =
        flume::bounded::<AudioBackendStateUpdate>(10);

    let _ = use_context_provider(|| command_tx);
    let _ = use_context_provider(|| command_rx);
    let _ = use_context_provider(|| notification_tx);
    let _ = use_context_provider(|| notification_rx);
    let _ = use_context_provider(|| audio_backend_state_tx);
    let _ = use_context_provider(|| audio_backend_state_rx);
    let _ = use_context_provider(LoginState::default);
}
