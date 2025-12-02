use crate::{PlayerCommand, model::Song};
use dioxus::{
    prelude::{Store, *},
    stores::hashmap::GetWrite,
};
use flume::Sender;
use std::{collections::HashMap, option::Option, time::Duration};

pub static CONTROLLER: GlobalStore<Controller> = Global::new(Controller::default);

type SongStore = Store<(bool, Song), GetWrite<usize, WriteSignal<HashMap<usize, (bool, Song)>>>>;

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
    current_idx: usize,
    sender: Option<Sender<PlayerCommand>>,
    current_song_id: Option<String>,
    queue_store: Store<HashMap<usize, (bool, Song)>>,
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

    fn current(
        &self,
    ) -> Option<SongStore>
    {
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
        self.queue(song);
        let len = self.queue_store().read().len();
        self.current_idx().set(len - 1);
        self.send(PlayerCommand::QueueNow(id));
    }

    fn queue(&mut self, song: Song) {
        tracing::info!("Queueing song: {}", song.title);
        let len = self.queue_store().read().len();
        self.queue_store().write().insert(len, (false, song));
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
        self.toggle_selected();
        *idx.write() += 1;
        let idx = *idx.read();
        self.toggle_selected();
        let song = self.queue_store().read().get(idx);
        if let Some(song) = song {
            self.current_song_id().set(Some(song.read().1.id.clone()));
        }
    }

    fn decrement_current(&mut self) {
        let mut idx = self.current_idx();
        if *idx.read() > 0 {
            self.toggle_selected();
            *idx.write() -= 1;
        }
        let idx = *idx.read();
        self.toggle_selected();
        let song = self.queue_store().read().get(idx);
        if let Some(song) = song {
            self.current_song_id().set(Some(song.read().1.id.clone()));
        }
    }

    fn remains(&self) -> usize {
        let current = *self.current_idx().read();
        self.queue_store().read().len() - current
    }

    fn toggle_selected(&mut self) {
        let idx = *self.current_idx().read();
        if let Some(mut store) = self.queue_store().write().get(idx) {
            let is_selected = store.read().0;
            store.write().0 = !is_selected;
        }
    }
}
