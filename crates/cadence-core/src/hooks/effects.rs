use crate::{
    player::{AudioBackend, AudioBackendStateUpdate},
    state::{CONTROLLER, ControllerExt, ControllerStoreExt, HostNotificationCommand},
};
use dioxus::prelude::*;

use crate::{
    hooks::use_saved_credentials,
    services::subsonic_client::{SUBSONIC_CLIENT, SubsonicClient},
};

pub fn use_audio_backend(ready: Signal<bool>, mut logged_in: Signal<bool>) {
    use_effect(move || {
        let _ = ready();
        if let Some(credentials) = use_saved_credentials()() {
            tracing::info!("Saved credentials changed");
            let client = SubsonicClient::new(
                &credentials.server_url,
                &credentials.username,
                &credentials.password,
            );

            *SUBSONIC_CLIENT.write() = Some(client.clone());

            spawn(async move {
                match client.ping().await {
                    Ok(ok) if ok => {
                        let mut player = AudioBackend::build(
                            &credentials.server_url,
                            &credentials.username,
                            &credentials.password,
                            use_context(),
                            use_context(),
                        )
                        .expect("Player start failed");
                        logged_in.set(true);
                        CONTROLLER.resolve().attach_sender(use_context());
                        if let Err(err) = player.run().await {
                            error!("Audio backend error: {}", err);
                        }
                    }
                    Ok(ko) => {
                        error!("failed to ping subsonic server");
                    }
                    Err(err) => {
                        error!("subsonic server error: {}", err);
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

pub fn use_audio_backend_state_update() {
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
