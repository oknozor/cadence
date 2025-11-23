use cadence_player::CadencePlayer;
use dioxus::prelude::*;

use crate::{
    hooks::{use_command_receiver, use_playback_position_sender, use_saved_credentials},
    services::subsonic_client::{SUBSONIC_CLIENT, SubsonicClient},
    state::LoginState,
};

pub fn use_on_login_effect() {
    let saved_credentials = use_saved_credentials();

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
                        spawn(async move {
                            let mut player = CadencePlayer::build(
                                &credentials.server_url,
                                &credentials.username,
                                &credentials.password,
                                use_command_receiver(),
                                use_playback_position_sender(),
                            )
                            .expect("Player start failed");
                            player.run().await.expect("Player run error");
                        });
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
