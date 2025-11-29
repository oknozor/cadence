mod backend;
pub use backend::{AudioBackend, MusicPlayerError};

mod notification;
pub use notification::NotificationControl;

use std::time::Duration;
use tracing::info;

use crate::model::Song;

#[derive(Debug, Clone)]
pub enum PlayerCommand {
    Play,
    Queue(Song),
    QueueNow(Song),
    QueueAll(Vec<Song>),
    Pause,
    Next,
    Previous,
    Seek(Duration),
}

impl AudioBackend {
    pub async fn run(&mut self) -> Result<(), MusicPlayerError> {
        info!("Audio backend starting...");
        let rx = self.command_rx.clone();
        tracing::error!("receiver has disconnected: {:?}", rx.is_disconnected());
        loop {
            tokio::select! {
                _ =  dioxus_sdk::time::sleep(Duration::from_secs(1)) => {
                    if let Ok(pos) = self.get_pos().await {
                        self.tx.send(pos).unwrap();
                    }
                },
                Ok(command) = rx.recv_async() => {
                    match command {
                        PlayerCommand::Play => self.play()?,
                        PlayerCommand::Queue(song) => self.queue(song).await?,
                        PlayerCommand::QueueAll(tracks) => self.queue_all_now(tracks).await?,
                        PlayerCommand::QueueNow(song) => {
                            self.queue_now(song).await?;
                        }
                        PlayerCommand::Pause => self.pause()?,
                        PlayerCommand::Seek(duration) => self.seek(duration)?,
                        PlayerCommand::Next => self.next()?,
                        PlayerCommand::Previous => self.previous().await?,
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
