mod backend;
pub use backend::{CadencePlayer, MusicPlayerError};

mod notification;
pub use notification::NotificationControl;

use std::time::Duration;
use tracing::info;

#[derive(Debug, Clone)]
pub enum PlayerCommand {
    Play,
    Queue(String),
    QueueNow {
        track_id: String,
        track_name: String,
        track_artist: String,
    },
    Pause,
    Next,
    Previous,
    Seek(Duration),
}

impl CadencePlayer {
    pub async fn run(&mut self) -> Result<(), MusicPlayerError> {
        info!("Cadence player running");
        let mut rx = self.rx.lock().await;

        loop {
            tokio::select! {
                _ =  dioxus_sdk::time::sleep(Duration::from_secs(1)) => {
                    if let Some(pos) = self.get_pos() {
                        self.tx.send(pos).unwrap();
                    }
                },
                Some(command) = rx.recv() => {
                    match command {
                        PlayerCommand::Play => self.play()?,
                        PlayerCommand::Queue(id) => self.queue(&id).await?,
                        PlayerCommand::QueueNow { track_id, track_name, track_artist } => {
                            if !self.is_empty() {
                                self.queue(&track_id).await?;
                                self.next()?
                            } else {
                                self.queue(&track_id).await?;
                            }

                            let _ = NotificationControl::update_media_notification(
                                       &track_name,
                                       &track_artist,
                                       123000,
                                       3000,
                                       !self.is_paused(),
                                       None,
                                   );
                        }

                        PlayerCommand::Pause => self.pause()?,
                        PlayerCommand::Seek(duration) => self.seek(duration)?,
                        PlayerCommand::Next => unimplemented!(),
                        PlayerCommand::Previous => unimplemented!(),
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
