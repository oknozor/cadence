use rodio::{Decoder, OutputStream, OutputStreamBuilder, Sink};
use std::{
    cell::RefCell,
    num::NonZeroUsize,
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicUsize, Ordering},
    },
    time::Duration,
};
use stream_download::{
    Settings, StreamDownload,
    source::DecodeError,
    storage::{adaptive::AdaptiveStorageProvider, temp::TempStorageProvider},
};
use tracing::info;

use crate::{
    PlayerCommand,
    model::Song,
    player::{AudioBackendStateUpdate, stream_url},
};

pub struct AudioBackend {
    sink: Sink,
    pub command_rx: flume::Receiver<PlayerCommand>,
    pub state_tx: flume::Sender<AudioBackendStateUpdate>,
    username: String,
    password: String,
    base_url: String,
    queue: RefCell<Vec<Song>>,
    current: Arc<AtomicUsize>,
    next_queued: Arc<AtomicBool>,
    _output_stream: OutputStream,
}

impl AudioBackend {
    pub fn build(
        base_url: impl ToString,
        username: impl ToString,
        password: impl ToString,
        command_rx: flume::Receiver<PlayerCommand>,
        state_tx: flume::Sender<AudioBackendStateUpdate>,
    ) -> Result<Self, MusicPlayerError> {
        let _output_stream = OutputStreamBuilder::open_default_stream()?;
        let sink = Sink::connect_new(_output_stream.mixer());
        let current = Arc::new(AtomicUsize::new(0));
        let next_queued = Arc::new(AtomicBool::new(false));

        Ok(AudioBackend {
            sink,
            username: username.to_string(),
            password: password.to_string(),
            base_url: base_url.to_string(),
            current,
            command_rx,
            _output_stream,
            state_tx,
            next_queued,
            queue: RefCell::new(vec![]),
        })
    }

    pub(crate) fn next(&self) -> Result<(), MusicPlayerError> {
        tracing::error!("Next functionality not implemented");
        self.sink.skip_one();
        Ok(())
    }

    pub(crate) async fn previous(&self) -> Result<(), MusicPlayerError> {
        let current = self.current.load(Ordering::Relaxed);
        if current == 0 {
            return Ok(());
        }

        self.current.store(current - 1, Ordering::Relaxed);
        let queue = self.queue.borrow();
        let track = queue.get(current - 1);
        if let Some(track) = track {
            self.open_stream(&track.id).await?;
            self.sink.play();
        }
        Ok(())
    }

    pub(crate) fn play(&self) -> Result<(), MusicPlayerError> {
        self.sink.play();
        Ok(())
    }

    pub(crate) fn pause(&self) -> Result<(), MusicPlayerError> {
        self.sink.pause();
        Ok(())
    }

    pub(crate) fn seek(&self, duration: Duration) -> Result<(), MusicPlayerError> {
        self.sink.try_seek(duration)?;
        Ok(())
    }

    pub(crate) async fn get_pos(&self) -> Result<u64, MusicPlayerError> {
        if !self.is_empty() {
            self.auto_queue().await?;
        }

        Ok(self.sink.get_pos().as_secs())
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.sink.empty()
    }

    pub async fn queue_all_now(&self, tracks: Vec<Song>) -> Result<(), MusicPlayerError> {
        let first_id = tracks.first().map(|t| t.id.clone()).unwrap();
        self.current.store(0, Ordering::Relaxed);

        let mut queue = self.queue.borrow_mut();
        queue.clear();
        queue.extend(tracks);

        self.sink.clear();
        self.sink.play();
        self.open_stream(&first_id).await?;

        Ok(())
    }

    pub(crate) async fn queue_now(&self, track: Song) -> Result<(), MusicPlayerError> {
        let id = track.id.clone();
        let mut queue = self.queue.borrow_mut();

        queue.clear();
        queue.push(track);
        self.sink.clear();
        self.sink.play();

        self.open_stream(&id).await?;
        Ok(())
    }

    pub(crate) async fn queue(&self, track: Song) -> Result<(), MusicPlayerError> {
        let mut queue = self.queue.borrow_mut();
        queue.push(track);
        Ok(())
    }

    async fn open_stream(&self, track_id: &str) -> Result<(), MusicPlayerError> {
        let username = &self.username;
        let password = &self.password;
        let base_url = &self.base_url;
        let url = stream_url(&base_url, username, password, &track_id);

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
        self.set_callback();
        Ok(())
    }

    // TODO: proper autoqueue implementation
    async fn auto_queue(&self) -> Result<(), MusicPlayerError> {
        if self.next_queued.load(Ordering::Relaxed) {
            return Ok(());
        }

        let queue = self.queue.borrow_mut();
        let current = self.current.load(Ordering::Relaxed);

        if let Some(track) = queue.get(current + 1) {
            let secs = self.sink.get_pos().as_secs_f64() as u64;
            let position = Duration::from_secs(secs);

            // TODO: we should not rely on track data for this
            if let Some(duration) = track.duration.map(|d| Duration::from_secs(d as u64)) {
                if (duration - position) < Duration::from_secs(5) {
                    info!("auto queued");
                    self.next_queued.store(true, Ordering::Relaxed);
                    self.open_stream(&track.id).await?;
                }
            }
        }

        Ok(())
    }

    fn set_callback(&self) {
        let playlist_pos = self.current.clone();
        let next_queued_clone = self.next_queued.clone();

        self.sink
            .append(rodio::source::EmptyCallback::new(Box::new(move || {
                playlist_pos.fetch_add(1, Ordering::Relaxed);
                next_queued_clone.store(false, Ordering::Relaxed);
            })));
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
