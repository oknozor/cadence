use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::components::album_card::Song;

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct SubSonicLogin {
    pub username: String,
    pub password: String,
    pub server_url: String,
}

#[derive(Default, Clone)]
pub struct Queue {
    current_track: Signal<Option<usize>>,
    queue: Signal<Vec<Song>>,
}

impl Queue {
    pub fn set_current(&mut self, song_id: &str) {
        let queue = self.queue.read();
        if let Some(idx) = queue.iter().position(|song| song.id == song_id) {
            self.current_track.set(Some(idx));
        }
    }

    pub fn get_current(&self) -> Option<Song> {
        let idx = *self.current_track.read();
        if let Some(idx) = idx {
            return self.queue.read().get(idx).cloned();
        }

        None
    }

    pub fn skip(&mut self) -> Option<Song> {
        let idx = *self.current_track.read();
        let queue_len = self.queue.read().len();
        if let Some(idx) = idx {
            if idx + 1 < queue_len {
                self.current_track.set(Some(idx + 1));
            } else {
                return None;
            }
        }

        self.get_current()
    }

    pub fn previous(&mut self) -> Option<Song> {
        let idx = *self.current_track.read();
        if let Some(idx) = idx {
            if idx == 0 {
                return None;
            }

            if idx > 0 {
                self.current_track.set(Some(idx - 1));
            }
        }

        self.get_current()
    }

    pub fn append_and_set_current(&mut self, song: Song) {
        self.queue.write().push(song);
        self.current_track.set(Some(self.queue.len() - 1));
    }

    pub fn append(&mut self, song: Song) {
        self.queue.write().push(song);
    }

    pub fn remove(&mut self, index: usize) {
        self.queue.write().remove(index);
    }
}

#[derive(Default, Clone)]
pub struct IsPlaying {
    playing: Signal<bool>,
    song_id: Signal<Option<String>>,
}

impl IsPlaying {
    pub fn song(&self) -> Option<String> {
        self.song_id.read().cloned()
    }

    pub fn is_playing(&self) -> bool {
        *self.playing.read()
    }

    pub fn to_signal(&self) -> Signal<bool> {
        self.playing.clone()
    }

    pub fn set_playing(&mut self, id: String) {
        self.playing.set(!self.is_playing());
        self.song_id.set(Some(id));
    }

    pub fn toggle(&mut self) {
        self.playing.set(!self.is_playing());
    }
}
