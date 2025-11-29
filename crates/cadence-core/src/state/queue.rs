use crate::{PlayerCommand, model::Song};
use dioxus::{
    prelude::{Store, *},
    stores::hashmap::GetWrite,
};
use flume::Sender;
use std::{cell::Ref, collections::HashMap, option::Option, time::Duration};

pub static CONTROLLER: GlobalStore<Controller> = Global::new(Controller::default);

#[derive(Debug)]
pub enum HostNotificationCommand {
    Play,
    Pause,
    Next,
    Previous,
    Seek(Duration),
}

#[derive(Store)]
pub struct Controller {
    pub is_playing: bool,
    pub shuffle: bool,
    pub position: Duration,
    sender: Option<Sender<PlayerCommand>>,
    current_idx: usize,
    current_song_id: Option<String>,
    queue_store: Store<HashMap<usize, Song>>,
}

impl Default for Controller {
    fn default() -> Self {
        Self {
            position: Duration::ZERO,
            current_idx: 0,
            is_playing: Default::default(),
            shuffle: Default::default(),
            queue_store: Store::new(HashMap::default()),
            sender: None,
            current_song_id: None,
        }
    }
}

#[store(pub name = ControllerExt)]
impl<Lens> Store<Controller, Lens> {
    fn attach_sender(&mut self, sender: Sender<PlayerCommand>) {
        *self.sender().write() = Some(sender);
    }

    fn current(&self) -> Option<Store<Song, GetWrite<usize, WriteSignal<HashMap<usize, Song>>>>> {
        let idx = *self.current_idx().read();
        self.queue_store().read().get(idx)
    }

    fn position_f64(&self) -> Option<f64> {
        Some(self.position().read().as_secs_f64())
    }

    fn seek(&mut self, pos: Duration) {
        self.position().set(pos);
        self.send(PlayerCommand::Seek(pos));
    }

    fn next(&mut self) {
        self.increment_current();
        if let Some(id) = self.current_song_id()() {
            self.send(PlayerCommand::QueueNow(id));
        }
    }

    fn previous(&mut self) {
        self.decrement_current();
        if let Some(id) = self.current_song_id()() {
            self.send(PlayerCommand::QueueNow(id));
        }
    }

    fn toggle_play(&mut self) {
        self.is_playing().toggle();
        if *self.is_playing().read() {
            self.send(PlayerCommand::Play);
        } else {
            self.send(PlayerCommand::Pause);
        }
    }

    fn play_now(&mut self, song: Song) {
        tracing::info!("Playing now song: {}", song.title);
        let id = song.id.clone();
        self.is_playing().set(true);
        self.current_song_id().set(Some(id.clone()));
        self.increment_current();
        self.queue(song);
        self.send(PlayerCommand::QueueNow(id));
    }

    fn queue(&mut self, song: Song) {
        tracing::info!("Queueing song: {}", song.title);
        let len = self.queue_store().read().len();
        self.queue_store().write().insert(len, song);
    }

    fn queue_all(&mut self, songs: Vec<Song>) {
        self.clear_queue();
        for (i, song) in songs.into_iter().enumerate() {
            if i == 0 {
                self.play_now(song);
            } else {
                self.queue(song);
            }
        }
    }

    fn on_playback_ended(&mut self) {
        if self.remains() > 0 {
            self.next();
        } else {
            self.toggle_play();
            self.position().set(Duration::ZERO);
        }
    }
}

#[store(name = ControllerStorePrivate)]
impl<Lens> Store<Controller, Lens> {
    fn sender_unchecked(&self) -> Sender<PlayerCommand> {
        self.sender().unwrap().cloned()
    }

    fn send(&self, command: PlayerCommand) {
        tracing::trace!("Sending player command: {:?}", command);
        if let Err(err) = self.sender_unchecked().send(command) {
            tracing::error!("failed to send message to audio backend: {}", err);
        }
    }

    fn clear_queue(&mut self) {
        self.queue_store().write().clear();
        self.current_idx().set(0);
        self.current_song_id().set(None);
    }

    fn increment_current(&mut self) {
        let mut idx = self.current_idx();
        *idx.write() += 1;
        let idx = *idx.read();
        let song = self.queue_store().read().get(idx);
        if let Some(song) = song {
            self.current_song_id().set(Some(song.read().id.clone()));
        }
    }

    fn decrement_current(&mut self) {
        let mut idx = self.current_idx();
        if *idx.read() > 0 {
            *idx.write() -= 1;
        }
        let idx = *idx.read();
        let song = self.queue_store().read().get(idx);
        if let Some(song) = song {
            self.current_song_id().set(Some(song.read().id.clone()));
        }
    }

    fn remains(&self) -> usize {
        let current = *self.current_idx().read();
        self.queue_store().read().len() - current
    }
}
