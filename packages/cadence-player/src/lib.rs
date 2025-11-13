use std::time::Duration;

#[cfg(target_arch = "wasm32")]
mod wasm;

#[cfg(target_arch = "wasm32")]
pub use wasm::{CadencePlayer, MusicPlayerError};

#[cfg(not(target_arch = "wasm32"))]
mod rodio;

#[cfg(not(target_arch = "wasm32"))]
pub use rodio::{CadencePlayer, MusicPlayerError};

pub enum PlayerCommand {
    Play,
    Queue(String),
    Pause,
    Seek(Duration),
}

impl CadencePlayer {
    pub async fn run(&mut self) -> Result<(), MusicPlayerError> {
        let mut rx = self.rx.lock().unwrap();
        while let Some(command) = rx.recv().await {
            match command {
                PlayerCommand::Play => self.play()?,
                PlayerCommand::Queue(id) => self.queue(&id).await?,
                PlayerCommand::Pause => self.pause()?,
                PlayerCommand::Seek(duration) => self.seek(duration)?,
            }
        }

        Ok(())
    }
}

fn stream_url(base: &str, user: &str, password: &str, id: &str) -> String {
    const BITRATE: u32 = 12000;
    const VERSION: &str = "1.16.1";
    format!(
        "{base}/rest/stream?estimatedContentLength=true&id={id}&maxBitrate={BITRATE}&u={user}&p={password}&v={VERSION}&c=cadence",
    )
}
