use crate::{PlayerCommand, model::Song};
use dioxus::prelude::*;
use flume::Sender;
use std::collections::HashMap;

pub static CONTROLLER: GlobalStore<Controller> = Global::new(Controller::default);

#[derive(Store)]
pub struct Controller {
    pub is_playing: bool,
    pub shuffle: bool,
    sender: Option<Sender<PlayerCommand>>,
    current_idx: Option<usize>,
    queue_store: Store<HashMap<usize, Song>>,
}

impl Default for Controller {
    fn default() -> Self {
        Self {
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

    fn toggle_play(&mut self) {
        self.is_playing().toggle();
        if *self.is_playing().read() {
            self.sender_unchecked()
                .send(PlayerCommand::Play)
                .expect("failed to send message to audio backend");
        } else {
            self.sender_unchecked()
                .send(PlayerCommand::Pause)
                .expect("failed to send message to audio backend");
        }
    }

    fn queue_now(&mut self, song: Song) {
        self.queue(song.clone());
        self.increment_current();
        info!("{:?}", self.sender_unchecked().receiver_count());
        if let Err(err) = self.sender_unchecked().send(PlayerCommand::QueueNow(song)) {
            tracing::error!("failed to send message to audio backend: {}", err);
        }
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
        self.queue_store()
            .read()
            .iter()
            .find(|(_, song)| song().id == song_id)
            .is_some()
    }
}

#[store(name = ControllerStorePrivate)]
impl<Lens> Store<Controller, Lens> {
    fn sender_unchecked(&self) -> Sender<PlayerCommand> {
        self.sender().unwrap().cloned()
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
