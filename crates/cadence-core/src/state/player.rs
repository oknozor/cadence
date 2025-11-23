use dioxus::signals::{ReadableExt, Signal, WritableExt};

#[derive(Default, Clone, PartialEq, Eq)]
pub struct PlayerState {
    playing: Signal<bool>,
    shuffle: Signal<bool>,
    song_id: Signal<Option<String>>,
}

impl PlayerState {
    pub fn song(&self) -> Option<String> {
        self.song_id.read().cloned()
    }

    pub fn is_playing(&self) -> Signal<bool> {
        self.playing.clone()
    }

    pub fn is_shuffle(&self) -> Signal<bool> {
        self.shuffle.clone()
    }

    pub fn set_playing(&mut self, id: String) {
        self.playing.set(true);
        self.song_id.set(Some(id));
    }

    pub fn toggle(&mut self) {
        let is_playing = *self.is_playing().read();
        self.playing.set(!is_playing);
    }
}
