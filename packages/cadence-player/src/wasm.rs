use crate::{PlayerCommand, stream_url};
use howler_wasm::JsHowl;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use web_time::Duration;

pub struct CadencePlayer {
    pub(super) rx: Arc<Mutex<mpsc::Receiver<PlayerCommand>>>,
    username: String,
    password: String,
    base_url: String,
    howler: RefCell<Option<JsHowl>>,
}

impl CadencePlayer {
    pub fn build(
        base_url: impl ToString,
        username: impl ToString,
        password: impl ToString,
        rx: Arc<Mutex<mpsc::Receiver<PlayerCommand>>>,
    ) -> Result<Self, MusicPlayerError> {
        howler_wasm::init();
        let howler = RefCell::new(None);

        Ok(CadencePlayer {
            username: username.to_string(),
            password: password.to_string(),
            base_url: base_url.to_string(),
            howler,
            rx,
        })
    }

    pub(super) fn play(&self) -> Result<(), MusicPlayerError> {
        Ok(())
    }

    pub(super) async fn queue(&self, id: &str) -> Result<(), MusicPlayerError> {
        let username = &self.username;
        let password = &self.password;
        let base_url = &self.base_url;
        let url = stream_url(base_url, username, password, id);

        if let Some(howl) = self.howler.take() {
            howl.unload();
        }

        let howl = JsHowl::new(url);
        howl.play();
        self.howler.borrow_mut().replace(howl);

        Ok(())
    }

    pub(super) fn pause(&self) -> Result<(), MusicPlayerError> {
        let borrow_mut = self.howler.borrow_mut();
        if let Some(howl) = borrow_mut.as_ref() {
            howl.pause();
        }

        Ok(())
    }

    pub(super) fn seek(&self, duration: Duration) -> Result<(), MusicPlayerError> {
        let borrow_mut = self.howler.borrow_mut();
        if let Some(howl) = borrow_mut.as_ref() {
            howl.seek(duration.as_secs());
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
