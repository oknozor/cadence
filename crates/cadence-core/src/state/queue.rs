use crate::{PlayerCommand, model::Song};
use dioxus::prelude::*;
use flume::Sender;
use std::{collections::HashMap, time::Duration};

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
    current_idx: Option<usize>,
    queue_store: Store<HashMap<usize, Song>>,
}

impl Default for Controller {
    fn default() -> Self {
        Self {
            position: Duration::ZERO,
            current_idx: Default::default(),
            is_playing: Default::default(),
            shuffle: Default::default(),
            queue_store: Store::new(HashMap::default()),
            sender: None,
        }
    }
}

#[store(pub name = ControllerExt)]
impl<Lens> Store<Controller, Lens> {
    fn attach_sender(&mut self, sender: Sender<PlayerCommand>) {
        *self.sender().write() = Some(sender);
    }

    fn current(&self) -> Option<Song> {
        self.current_idx()
            .read()
            .and_then(|idx: usize| self.queue_store().read().get(idx).map(|store| store()))
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
        self.send(PlayerCommand::Next);
    }

    fn previous(&mut self) {
        self.decrement_current();
        self.send(PlayerCommand::Previous);
    }

    fn toggle_play(&mut self) {
        self.is_playing().toggle();
        if *self.is_playing().read() {
            self.send(PlayerCommand::Play);
        } else {
            self.send(PlayerCommand::Pause);
        }
    }

    fn queue_now(&mut self, song: Song) {
        self.is_playing().set(true);
        self.queue(song.clone());
        self.increment_current();
        self.send(PlayerCommand::QueueNow(song));
    }

    fn queue(&mut self, song: Song) {
        let len = self.queue_store().read().len();
        self.queue_store().write().insert(len, song);
    }

    fn queue_all(&mut self, songs: Vec<Song>) {
        songs.into_iter().for_each(|song| {
            self.queue(song);
        });
    }

    fn is_active(&self, song_id: &str) -> bool {
        self.current()
            .map(|current| current.id == song_id)
            .unwrap_or_default()
    }

    fn finish(&mut self) {
        self.toggle_play();
        self.seek(Duration::ZERO);
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

    fn increment_current(&mut self) {
        match self.current_idx().transpose() {
            Some(mut idx) => {
                *idx.write() += 1;
            }
            None => self.current_idx().set(Some(0)),
        }
    }

    fn decrement_current(&mut self) {
        match self.current_idx().transpose() {
            Some(mut idx) => {
                if *idx.read() > 0 {
                    *idx.write() -= 1;
                }
            }
            None => self.current_idx().set(None),
        }
    }
}
