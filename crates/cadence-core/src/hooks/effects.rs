use crate::{
    player::{AudioBackend, AudioBackendStateUpdate},
    state::{CONTROLLER, ControllerExt, ControllerStoreExt, HostNotificationCommand},
};
use dioxus::prelude::*;

use crate::{
    hooks::use_saved_credentials,
    services::subsonic_client::{SUBSONIC_CLIENT, SubsonicClient},
    state::LoginState,
};

pub fn initialize_audio_backend() {
    let saved_credentials = use_saved_credentials();
    use_audio_backend_state_update();

    use_effect(move || {
        if let Some(credentials) = saved_credentials.read().cloned() {
            let client = SubsonicClient::new(
                &credentials.server_url,
                &credentials.username,
                &credentials.password,
            );

            *SUBSONIC_CLIENT.write() = Some(client.clone());

            let mut login_state = consume_context::<LoginState>();
            spawn(async move {
                match client.ping().await {
                    Ok(ok) if ok => {
                        info!("Logged in");
                        login_state.set(true);
                        let mut player = AudioBackend::build(
                            &credentials.server_url,
                            &credentials.username,
                            &credentials.password,
                            use_context(),
                            use_context(),
                        )
                        .expect("Player start failed");

                        CONTROLLER.resolve().attach_sender(use_context());
                        if let Err(err) = player.run().await {
                            error!("Audio backend error: {}", err);
                        }
                    }
                    Ok(ko) => {
                        login_state.set_error(Some(format!("Login failed: {}", ko)));
                    }
                    Err(err) => {
                        login_state.set_error(Some(format!("Login failed: {}", err)));
                    }
                }
            });
        }
    });
}

pub fn use_notification_control() {
    use_effect(move || {
        let rx: flume::Receiver<HostNotificationCommand> = use_context();
        spawn(async move {
            loop {
                match rx.recv_async().await {
                    Ok(command) => {
                        tracing::info!("Received host notification command: {command:?}");
                        match command {
                            HostNotificationCommand::Play | HostNotificationCommand::Pause => {
                                CONTROLLER.resolve().toggle_play()
                            }
                            HostNotificationCommand::Next => CONTROLLER.resolve().next(),
                            HostNotificationCommand::Previous => CONTROLLER.resolve().previous(),
                            HostNotificationCommand::Seek(duration) => {
                                CONTROLLER.resolve().seek(duration)
                            }
                        }
                    }
                    Err(err) => error!("failed to handle notification command: {err}"),
                }
            }
        });
    });
}

fn use_audio_backend_state_update() {
    use_effect(move || {
        let rx: flume::Receiver<AudioBackendStateUpdate> = use_context();

        spawn(async move {
            loop {
                let mut controller = CONTROLLER.resolve();

                match rx.recv_async().await {
                    Ok(AudioBackendStateUpdate::Position(pos)) => {
                        controller.position().set(pos);
                    }
                    Ok(AudioBackendStateUpdate::Finished) => {
                        tracing::info!("Audio backend finished");
                        controller.on_playback_ended();
                    }
                    Err(err) => error!("failed to handle playback position update: {err}"),
                }
            }
        });
    });
}
