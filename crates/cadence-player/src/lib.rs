#[cfg(target_arch = "wasm32")]
mod wasm;

#[cfg(target_os = "android")]
pub mod android_backend;

#[cfg(target_os = "android")]
pub use android_backend::*;

#[cfg(not(target_arch = "wasm32"))]
use std::time::Duration;
#[cfg(target_arch = "wasm32")]
use std::time::Duration;
use tracing::info;
#[cfg(target_arch = "wasm32")]
pub use wasm::{CadencePlayer, MusicPlayerError};

#[cfg(not(target_arch = "wasm32"))]
mod rodio;
#[cfg(not(target_arch = "wasm32"))]
pub use rodio::{CadencePlayer, MusicPlayerError};

pub enum PlayerCommand {
    Play,
    Queue(String),
    QueueNow(String),
    Pause,
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
                        PlayerCommand::QueueNow(id) => {
                            if !self.is_empty() {
                                self.queue(&id).await?;
                                self.next()?
                            } else {
                                self.queue(&id).await?;
                            }
                        }
                        PlayerCommand::Pause => self.pause()?,
                        PlayerCommand::Seek(duration) => self.seek(duration)?,
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
