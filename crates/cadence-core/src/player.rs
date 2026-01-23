mod backend;
pub use backend::{AudioBackend, MusicPlayerError};

mod notification;
pub use notification::NotificationControl;

use crate::model::RadioStation;
use std::time::Duration;
use tracing::info;

#[derive(Debug)]
pub enum PlayerCommand {
    Play,
    Pause,
    Next,
    Seek(Duration),
    Queue(String),
    QueueNow(String),
    PlayRadio(RadioStation),
}

#[derive(Debug)]
pub enum AudioBackendStateUpdate {
    Finished,
    Position(Duration),
}

impl AudioBackend {
    pub async fn run(&mut self) -> Result<(), MusicPlayerError> {
        info!("Audio backend starting...");
        let rx = self.command_rx.clone();

        loop {
            tokio::select! {
                _ =  dioxus_sdk::time::sleep(Duration::from_millis(200)) => {
                    if let Ok(pos) = self.get_pos().await {
                        self.state_tx.send_async(AudioBackendStateUpdate::Position(Duration::from_secs(pos))).await.unwrap();
                    }
                },
                Ok(command) = rx.recv_async() => {
                    tracing::info!("Received command: {:?}", command);
                    match command {
                        PlayerCommand::Queue(id) => self.queue(&id).await?,
                        PlayerCommand::QueueNow(id) => {
                            self.queue_now(&id).await?;
                        }
                        PlayerCommand::Play => self.play()?,
                        PlayerCommand::Pause => self.pause()?,
                        PlayerCommand::Next => self.next()?,
                        PlayerCommand::Seek(duration) => self.seek(duration)?,
                        PlayerCommand::PlayRadio(radio) => self.play_radio(&radio).await?,
                    }
                }
            }
        }
    }
}

fn stream_url(base: &str, user: &str, password: &str, id: &str) -> String {
    const BITRATE: u32 = 12000;
    const VERSION: &str = "1.16.1";
    format!(
        "{base}/rest/stream?estimatedContentLength=true&id={id}&maxBitrate={BITRATE}&u={user}&p={password}&v={VERSION}&c=cadence",
    )
}
