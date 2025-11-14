use dioxus::prelude::*;

use crate::album_card::Song;

#[derive(Default, Clone)]
pub struct Queue {
    pub current_track: Signal<Option<usize>>,
    pub queue: Signal<Vec<Song>>,
}

#[derive(Default, Clone)]
pub struct CurrentTrack {
    pub track: Signal<Option<Song>>,
}

#[derive(Default, Clone)]
pub struct IsPlaying(Signal<bool>);

impl IsPlaying {
    pub fn is_playing(&self) -> bool {
        *self.0.read()
    }

    pub fn toggle(&mut self) {
        self.0.set(!self.is_playing());
    }
}
