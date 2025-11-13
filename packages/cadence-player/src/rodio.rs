use crate::{PlayerCommand, stream_url};
use rodio::{Decoder, OutputStream, OutputStreamBuilder, Sink};
use std::{
    num::NonZeroUsize,
    sync::{Arc, Mutex},
    time::Duration,
};
use stream_download::{
    Settings, StreamDownload,
    source::DecodeError,
    storage::{adaptive::AdaptiveStorageProvider, temp::TempStorageProvider},
};
use tokio::sync::mpsc;

pub struct CadencePlayer {
    sink: Sink,
    pub(super) rx: Arc<Mutex<mpsc::Receiver<PlayerCommand>>>,
    username: String,
    password: String,
    base_url: String,
    _output_stream: OutputStream,
}

impl CadencePlayer {
    pub fn build(
        base_url: impl ToString,
        username: impl ToString,
        password: impl ToString,
        rx: Arc<Mutex<mpsc::Receiver<PlayerCommand>>>,
    ) -> Result<Self, MusicPlayerError> {
        let _output_stream = OutputStreamBuilder::open_default_stream()?;
        let sink = Sink::connect_new(&_output_stream.mixer());

        Ok(CadencePlayer {
            sink,
            username: username.to_string(),
            password: password.to_string(),
            base_url: base_url.to_string(),
            rx,
            _output_stream,
        })
    }

    pub(super) fn play(&self) -> Result<(), MusicPlayerError> {
        self.sink.play();
        Ok(())
    }

    pub(super) async fn queue(&self, id: &str) -> Result<(), MusicPlayerError> {
        let username = &self.username;
        let password = &self.password;
        let base_url = &self.base_url;
        let url = stream_url(base_url, username, password, id);
        self.open_stream(url).await?;
        Ok(())
    }

    pub(super) fn pause(&self) -> Result<(), MusicPlayerError> {
        self.sink.pause();
        Ok(())
    }

    pub(super) fn seek(&self, duration: Duration) -> Result<(), MusicPlayerError> {
        self.sink.try_seek(duration)?;
        Ok(())
    }

    async fn open_stream(&self, url: String) -> Result<(), MusicPlayerError> {
        let reader = match StreamDownload::new_http(
            url.parse()?,
            AdaptiveStorageProvider::new(
                TempStorageProvider::default(),
                NonZeroUsize::new(512 * 1024).unwrap(),
            ),
            Settings::default(),
        )
        .await
        {
            Ok(reader) => reader,
            Err(e) => return Err(MusicPlayerError::Custom(e.decode_error().await)),
        };
        let source = Decoder::new(reader)?;
        self.sink.append(source);
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum MusicPlayerError {
    #[error("play error: {0}")]
    Play(#[from] rodio::PlayError),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Stream error: {0}")]
    Stream(#[from] rodio::StreamError),
    #[error("Parse error: {0}")]
    Parse(#[from] url::ParseError),
    #[error("ParseInt error: {0}")]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("Decode error: {0}")]
    Decode(#[from] rodio::decoder::DecoderError),
    #[error("Join error: {0}")]
    Join(#[from] tokio::task::JoinError),
    #[error("Seek error: {0}")]
    Seek(#[from] rodio::source::SeekError),
    #[error("Custom error: {0}")]
    Custom(String),
}
