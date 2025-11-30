use crate::PlayerCommand;
use crate::player::{AudioBackendStateUpdate, stream_url};
use howler_wasm::JsHowl;
use std::cell::RefCell;
use std::collections::VecDeque;
use web_time::Duration;

pub struct AudioBackend {
    pub command_rx: flume::Receiver<PlayerCommand>,
    pub state_tx: flume::Sender<AudioBackendStateUpdate>,
    username: String,
    password: String,
    base_url: String,
    queue: RefCell<VecDeque<JsHowl>>,
}

impl AudioBackend {
    pub fn build(
        base_url: impl ToString,
        username: impl ToString,
        password: impl ToString,
        command_rx: flume::Receiver<PlayerCommand>,
        state_tx: flume::Sender<AudioBackendStateUpdate>,
    ) -> Result<Self, MusicPlayerError> {
        howler_wasm::init();

        Ok(AudioBackend {
            username: username.to_string(),
            password: password.to_string(),
            base_url: base_url.to_string(),
            queue: RefCell::new(VecDeque::new()),
            command_rx,
            state_tx,
        })
    }

    pub fn play(&self) -> Result<(), MusicPlayerError> {
        if let Some(howl) = self.queue.borrow_mut().back() {
            howl.play();
        }
        Ok(())
    }

    pub async fn queue_now(&self, id: &str) -> Result<(), MusicPlayerError> {
        let username = &self.username;
        let password = &self.password;
        let base_url = &self.base_url;
        let url = stream_url(base_url, username, password, id);

        let howl = JsHowl::new(url);
        if self.is_empty() {
            tracing::debug!("queue is empty, playing now");
            howl.play();
        }

        self.queue.borrow_mut().push_back(howl);
        Ok(())
    }

    pub async fn queue(&self, id: &str) -> Result<(), MusicPlayerError> {
        let username = &self.username;
        let password = &self.password;
        let base_url = &self.base_url;
        let url = stream_url(base_url, username, password, id);

        let howl = JsHowl::new(url);
        if self.is_empty() {
            tracing::debug!("queue is empty, playing now");
            howl.play();
        }

        self.queue.borrow_mut().push_back(howl);
        Ok(())
    }

    pub fn pause(&self) -> Result<(), MusicPlayerError> {
        let queue = self.queue.borrow_mut();
        let current = queue.back();

        if let Some(howl) = current.as_ref() {
            howl.pause();
        }

        Ok(())
    }

    pub fn seek(&self, duration: Duration) -> Result<(), MusicPlayerError> {
        let queue = self.queue.borrow_mut();
        let current = queue.back();

        if let Some(howl) = current.as_ref() {
            howl.seek(duration.as_secs());
        }
        Ok(())
    }

    pub(super) fn is_empty(&self) -> bool {
        self.queue.borrow().is_empty()
    }

    pub fn next(&self) -> Result<(), MusicPlayerError> {
        if self.is_empty() {
            return Ok(());
        }

        let mut queue = self.queue.borrow_mut();

        if let Some(howl) = queue.pop_front() {
            howl.stop();
            howl.unload();
        }

        if let Some(howl) = queue.back() {
            howl.play();
        }

        Ok(())
    }

    pub async fn get_pos(&self) -> Result<u64, MusicPlayerError> {
        let queue = self.queue.borrow_mut();
        queue
            .back()
            .map(|howl| howl.position())
            .ok_or(MusicPlayerError::Custom(
                "Failed to get player position".to_string(),
            ))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum MusicPlayerError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parse error: {0}")]
    Parse(#[from] url::ParseError),
    #[error("ParseInt error: {0}")]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("Join error: {0}")]
    Join(#[from] tokio::task::JoinError),
    #[error("Custom error: {0}")]
    Custom(String),
}
