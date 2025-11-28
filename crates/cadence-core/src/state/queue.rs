use dioxus::signals::{ReadableExt, Signal, WritableExt};

use crate::model::Song;

#[derive(Default, Clone)]
pub struct QueueState {
    current_track_idx: Signal<Option<usize>>,
    songs: Signal<Vec<Song>>,
}

impl QueueState {
    pub fn current_track_idx(&self) -> Option<usize> {
        *self.current_track_idx.read()
    }

    pub fn songs(&self) -> Vec<Song> {
        self.songs.read().cloned()
    }

    pub fn queue_all(&mut self, songs: Vec<Song>) {
        if songs.is_empty() {
            return;
        }

        self.songs.write().extend(songs);
        self.current_track_idx.set(Some(0));
    }

    pub fn append_and_set_current(&mut self, song: Song) {
        self.songs.write().push(song);
        self.current_track_idx
            .set(Some(self.songs.read().len() - 1));
    }
}
