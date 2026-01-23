use rodio::{Decoder, OutputStream, OutputStreamBuilder, Sink};
use std::{num::NonZeroUsize, time::Duration};
use stream_download::{
    source::DecodeError, storage::{
        adaptive::AdaptiveStorageProvider, memory::MemoryStorageProvider, temp::TempStorageProvider,
    },
    Settings,
    StreamDownload,
};

#[cfg(all(target_os = "android", feature = "mobile"))]
use cadence_storage_android;

use crate::model::RadioStation;
use crate::{
    player::{stream_url, AudioBackendStateUpdate},
    PlayerCommand,
};

pub struct AudioBackend {
    pub command_rx: flume::Receiver<PlayerCommand>,
    pub state_tx: flume::Sender<AudioBackendStateUpdate>,
    sink: Sink,
    username: String,
    password: String,
    base_url: String,
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

        Ok(AudioBackend {
            sink,
            username: username.to_string(),
            password: password.to_string(),
            base_url: base_url.to_string(),
            command_rx,
            _output_stream,
            state_tx,
        })
    }

    pub(crate) fn next(&self) -> Result<(), MusicPlayerError> {
        tracing::error!("Next functionality not implemented");
        self.sink.skip_one();
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
        tracing::info!("Seeking to {duration:?}");
        self.sink.try_seek(duration)?;
        Ok(())
    }

    pub(crate) async fn get_pos(&self) -> Result<u64, MusicPlayerError> {
        Ok(self.sink.get_pos().as_secs())
    }

    pub(crate) async fn queue_now(&self, track_id: &str) -> Result<(), MusicPlayerError> {
        tracing::info!("Queuing now {track_id}");
        self.sink.clear();
        self.sink.play();

        self.open_stream(track_id).await
    }

    pub(crate) async fn queue(&self, track_id: &str) -> Result<(), MusicPlayerError> {
        tracing::info!("Queuing {track_id}");
        self.open_stream(track_id).await
    }

    pub(crate) async fn play_radio(&self, radio: &RadioStation) -> Result<(), MusicPlayerError> {
        tracing::info!("Playing radio stream: {}", radio.stream_url);
        self.sink.clear();
        self.sink.play();

        self.open_radio_stream(&radio.stream_url).await
    }

    async fn open_stream(&self, track_id: &str) -> Result<(), MusicPlayerError> {
        let username = &self.username;
        let password = &self.password;
        let base_url = &self.base_url;
        let url = stream_url(base_url, username, password, track_id);

        let reader = match StreamDownload::new_http(
            url.parse()?,
            AdaptiveStorageProvider::new(
                // FIXME: Here we need temp storage provider
                //  this works fine for native/browser, but for android we need
                //  to manually set the path to the non priviledged app cache dir
                MemoryStorageProvider,
                // 64 MB Buffer Size, if we have a bigger content length, seek won't be avalaible
                NonZeroUsize::new(64 * 1024 * 1024).unwrap(),
            ),
            Settings::default(),
        )
            .await
        {
            Ok(reader) => reader,
            Err(e) => return Err(MusicPlayerError::Custom(e.decode_error().await)),
        };

        let decoder = if let Some(lenght) = reader.content_length() {
            rodio::decoder::DecoderBuilder::default()
                .with_seekable(true)
                .with_byte_len(lenght)
                .with_data(reader)
                .build()
        } else {
            Decoder::new(reader)
        }
            .expect("Failed to build audiobackend decoder");

        self.sink.append(decoder);
        self.set_callback();
        Ok(())
    }

    async fn open_radio_stream(&self, stream_url: &str) -> Result<(), MusicPlayerError> {
        // On Android, use the app's cache directory for temp files
        // On other platforms, use the default system temp directory
        #[cfg(all(target_os = "android", feature = "mobile"))]
        let storage_provider = TempStorageProvider::new_in(cadence_storage_android::cache_dir());

        #[cfg(not(all(target_os = "android", feature = "mobile")))]
        let storage_provider = TempStorageProvider::default();

        // Use a small prefetch for radio streams to minimize startup delay.
        // Default is 256KB which causes ~10+ second delays on live streams.
        // 8KB is enough for the decoder to start while keeping latency low.
        let settings = Settings::default().prefetch_bytes(8 * 1024);

        let reader = match StreamDownload::new_http(
            stream_url.parse()?,
            storage_provider,
            settings,
        )
            .await
        {
            Ok(reader) => reader,
            Err(e) => return Err(MusicPlayerError::Custom(e.decode_error().await)),
        };

        let decoder = Decoder::new(reader).map_err(|e| {
            MusicPlayerError::Custom(format!("Failed to build radio decoder: {}", e))
        })?;

        self.sink.append(decoder);
        Ok(())
    }

    fn set_callback(&self) {
        tracing::info!("Sink callback, queue len: {}", self.sink.len());
        let sender = self.state_tx.clone();

        self.sink
            .append(rodio::source::EmptyCallback::new(Box::new(move || {
                sender
                    .send(AudioBackendStateUpdate::Finished)
                    .expect("Audio backend failed to notifiy controller");
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

#[cfg(test)]
mod test {}
