use crate::{PlayerCommand, stream_url};
use howler_wasm::JsHowl;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::{Mutex, mpsc};
use web_time::Duration;

pub struct CadencePlayer {
    pub(super) rx: Arc<Mutex<mpsc::Receiver<PlayerCommand>>>,
    username: String,
    password: String,
    base_url: String,
    queue: RefCell<VecDeque<JsHowl>>,
}

impl CadencePlayer {
    pub fn build(
        base_url: impl ToString,
        username: impl ToString,
        password: impl ToString,
        rx: Arc<Mutex<mpsc::Receiver<PlayerCommand>>>,
    ) -> Result<Self, MusicPlayerError> {
        howler_wasm::init();

        Ok(CadencePlayer {
            username: username.to_string(),
            password: password.to_string(),
            base_url: base_url.to_string(),
            queue: RefCell::new(VecDeque::new()),
            rx,
        })
    }

    pub(super) fn play(&self) -> Result<(), MusicPlayerError> {
        if let Some(howl) = self.queue.borrow_mut().back() {
            howl.play();
        }
        Ok(())
    }

    pub(super) async fn queue(&self, id: &str) -> Result<(), MusicPlayerError> {
        let username = &self.username;
        let password = &self.password;
        let base_url = &self.base_url;
        let url = stream_url(base_url, username, password, id);

        let howl = JsHowl::new(url);
        if self.is_empty() {
            howl.play();
        }

        self.queue.borrow_mut().push_back(howl);
        Ok(())
    }

    pub(super) fn pause(&self) -> Result<(), MusicPlayerError> {
        let queue = self.queue.borrow_mut();
        let current = queue.back();

        if let Some(howl) = current.as_ref() {
            howl.pause();
        }

        Ok(())
    }

    pub(super) fn seek(&self, duration: Duration) -> Result<(), MusicPlayerError> {
        let mut queue = self.queue.borrow_mut();
        let current = queue.back();

        if let Some(howl) = current.as_ref() {
            howl.seek(duration.as_secs());
        }
        Ok(())
    }

    pub(super) fn is_empty(&self) -> bool {
        self.queue.borrow().is_empty()
    }

    pub(super) fn next(&self) -> Result<(), MusicPlayerError> {
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
