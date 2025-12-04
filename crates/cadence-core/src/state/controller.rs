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
    pub random: bool,
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
            random: false,
        }
    }
}

#[store(pub name = ControllerExt)]
impl<Lens> Store<Controller, Lens> {
    fn attach_sender(&mut self, sender: Sender<PlayerCommand>) {
        *self.sender().write() = Some(sender);
    }

    fn current(&self) -> Option<SongStore> {
        let idx = *self.current_idx().read();
        self.queue_store().read().get(idx)
    }

    fn position_f64(&self) -> Option<f64> {
        Some(self.position().read().as_secs_f64())
    }

    fn position_display(&self) -> String {
        let duration = *self.position().read();
        let minutes = duration.as_secs() / 60;
        let seconds = duration.as_secs() % 60;
        format!("{:02}:{:02}", minutes, seconds)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::player::PlayerCommand;
    use dioxus::core::Runtime;
    use dioxus::prelude::*;
    #[component]
    fn TestApp() -> Element {
        rsx! {}
    }

    fn start_runtime() {
        dioxus::launch(TestApp);
    }

    fn song(id: &str, title: &str) -> Song {
        Song {
            id: id.to_string(),
            title: title.to_string(),
            artist: String::new(),
            album: String::new(),
            cover_art: None,
            track_number: None,
            duration: None,
        }
    }

    fn setup() -> (
        Store<Controller, dioxus::prelude::WriteSignal<Controller>>,
        flume::Receiver<PlayerCommand>,
    ) {
        let mut controller: Store<Controller, dioxus::prelude::WriteSignal<Controller>> =
            Store::new(Controller::default());
        let (tx, rx) = flume::unbounded();
        controller.attach_sender(tx);
        (controller, rx)
    }

    #[test]
    fn queue_adds_without_playing() {
        let (mut controller, rx) = setup();
        controller.queue(song("1", "a"));
        controller.queue(song("2", "b"));
        assert_eq!(controller.queue_store().read().len(), 2);
        assert_eq!(*controller.current_idx().read(), 0);
        assert_eq!(*controller.is_playing().read(), false);
        assert!(rx.try_recv().is_err());
    }

    #[test]
    fn play_now_sets_current_and_sends_queue_now() {
        let (mut controller, rx) = setup();
        controller.play_now(song("42", "x"));
        assert_eq!(*controller.is_playing().read(), true);
        assert_eq!(controller.current_song_id()(), Some("42".to_string()));
        assert_eq!(controller.queue_store().read().len(), 1);
        assert_eq!(*controller.current_idx().read(), 0);
        let cmd = rx.try_recv().unwrap();
        match cmd {
            PlayerCommand::QueueNow(id) => assert_eq!(id, "42"),
            _ => panic!(),
        }
    }

    #[test]
    fn queue_all_resets_and_starts_first() {
        let (mut controller, rx) = setup();
        controller.queue(song("old", "o"));
        controller.queue_all(vec![song("1", "a"), song("2", "b"), song("3", "c")]);
        assert_eq!(controller.queue_store().read().len(), 3);
        assert_eq!(*controller.current_idx().read(), 0);
        assert_eq!(controller.current_song_id()(), Some("1".to_string()));
        assert_eq!(*controller.is_playing().read(), true);
        let cmd = rx.try_recv().unwrap();
        match cmd {
            PlayerCommand::QueueNow(id) => assert_eq!(id, "1"),
            _ => panic!(),
        }
    }

    #[test]
    fn next_moves_and_sends_queue_now() {
        let (mut controller, rx) = setup();
        controller.queue_all(vec![song("1", "a"), song("2", "b"), song("3", "c")]);
        let _ = rx.try_recv();
        controller.next();
        assert_eq!(*controller.current_idx().read(), 1);
        assert_eq!(controller.current_song_id()(), Some("2".to_string()));
        let cmd = rx.try_recv().unwrap();
        match cmd {
            PlayerCommand::QueueNow(id) => assert_eq!(id, "2"),
            _ => panic!(),
        }
    }

    #[test]
    fn previous_moves_and_sends_queue_now() {
        let (mut controller, rx) = setup();
        controller.queue_all(vec![song("1", "a"), song("2", "b")]);
        let _ = rx.try_recv();
        controller.next();
        let _ = rx.try_recv();
        controller.previous();
        assert_eq!(*controller.current_idx().read(), 0);
        assert_eq!(controller.current_song_id()(), Some("1".to_string()));
        let cmd = rx.try_recv().unwrap();
        match cmd {
            PlayerCommand::QueueNow(id) => assert_eq!(id, "1"),
            _ => panic!(),
        }
    }

    #[test]
    fn seek_updates_position_and_sends() {
        let (mut controller, rx) = setup();
        let d = Duration::from_secs(15);
        controller.seek(d);
        assert_eq!(*controller.position().read(), d);
        let cmd = rx.try_recv().unwrap();
        match cmd {
            PlayerCommand::Seek(x) => assert_eq!(x, d),
            _ => panic!(),
        }
    }

    #[test]
    fn toggle_play_sends_commands() {
        let (mut controller, rx) = setup();
        controller.toggle_play();
        assert_eq!(*controller.is_playing().read(), true);
        match rx.try_recv().unwrap() {
            PlayerCommand::Play => {}
            _ => panic!(),
        }
        controller.toggle_play();
        assert_eq!(*controller.is_playing().read(), false);
        match rx.try_recv().unwrap() {
            PlayerCommand::Pause => {}
            _ => panic!(),
        }
    }

    #[test]
    fn on_playback_ended_behaviour() {
        let (mut controller, rx) = setup();
        controller.queue_all(vec![song("1", "a"), song("2", "b")]);
        let _ = rx.try_recv();
        controller.on_playback_ended();
        let cmd = rx.try_recv().unwrap();
        match cmd {
            PlayerCommand::QueueNow(id) => assert_eq!(id, "2"),
            _ => panic!(),
        }
    }

    #[test]
    fn previous_at_start_resends_same_song() {
        let (mut controller, rx) = setup();
        controller.queue_all(vec![song("1", "a"), song("2", "b")]);
        let _ = rx.try_recv();
        controller.previous();
        assert_eq!(*controller.current_idx().read(), 0);
        assert_eq!(controller.current_song_id()(), Some("1".to_string()));
        match rx.try_recv().unwrap() {
            PlayerCommand::QueueNow(id) => assert_eq!(id, "1"),
            _ => panic!(),
        }
    }

    #[test]
    fn remains_calculation_and_selection_flags() {
        let (mut controller, rx) = setup();
        controller.queue_all(vec![song("1", "a"), song("2", "b"), song("3", "c")]);
        let _ = rx.try_recv();
        assert_eq!(controller.remains(), 3);
        controller.next();
        let _ = rx.try_recv();
        assert_eq!(controller.remains(), 2);
        {
            let s0 = controller.queue_store().read().get(0).unwrap();
            let s1 = controller.queue_store().read().get(1).unwrap();
            assert_eq!(s0.read().0, true);
            assert_eq!(s1.read().0, true);
        }
        controller.previous();
        let _ = rx.try_recv();
        {
            let s0 = controller.queue_store().read().get(0).unwrap();
            let s1 = controller.queue_store().read().get(1).unwrap();
            assert_eq!(s0.read().0, false);
            assert_eq!(s1.read().0, false);
        }
    }

    #[test]
    fn on_playback_ended_at_last_advances_out_of_bounds_current_behavior() {
        let (mut controller, rx) = setup();
        controller.queue_all(vec![song("1", "a"), song("2", "b")]);
        let _ = rx.try_recv();
        controller.next();
        let _ = rx.try_recv();
        assert_eq!(*controller.current_idx().read(), 1);
        controller.on_playback_ended();
        assert_eq!(*controller.current_idx().read(), 2);
        assert_eq!(controller.current_song_id()(), None);
        assert!(rx.try_recv().is_err());
    }

    #[test]
    fn clear_queue_resets_state() {
        let (mut controller, _rx) = setup();
        controller.queue_all(vec![song("1", "a"), song("2", "b")]);
        controller.clear_queue();
        assert_eq!(controller.queue_store().read().len(), 0);
        assert_eq!(*controller.current_idx().read(), 0);
        assert_eq!(controller.current_song_id()(), None);
        assert_eq!(*controller.is_playing().read(), true);
    }

    #[test]
    fn queue_after_play_now_keeps_current() {
        let (mut controller, rx) = setup();
        controller.play_now(song("1", "a"));
        let _ = rx.try_recv();
        controller.queue(song("2", "b"));
        controller.queue(song("3", "c"));
        assert_eq!(controller.queue_store().read().len(), 3);
        assert_eq!(*controller.current_idx().read(), 0);
        assert_eq!(controller.current_song_id()(), Some("1".to_string()));
    }

    #[test]
    fn current_returns_correct_song_store() {
        let (mut controller, rx) = setup();
        controller.queue_all(vec![song("1", "a"), song("2", "b")]);
        let _ = rx.try_recv();
        let cur = controller.current().unwrap();
        assert_eq!(cur.read().1.id, "1");
        controller.next();
        let _ = rx.try_recv();
        let cur = controller.current().unwrap();
        assert_eq!(cur.read().1.id, "2");
    }

    #[test]
    fn current_song_id_none_until_play_now_or_navigation() {
        let (mut controller, _rx) = setup();
        controller.queue(song("1", "a"));
        controller.queue(song("2", "b"));
        assert_eq!(controller.current_song_id()(), None);
        controller.play_now(song("3", "c"));
        assert_eq!(controller.current_song_id()(), Some("3".to_string()));
    }

    #[test]
    fn next_previous_do_not_send_when_no_current() {
        let (mut controller, rx) = setup();
        controller.next();
        assert!(rx.try_recv().is_err());
        controller.previous();
        assert!(rx.try_recv().is_err());
    }

    #[test]
    fn queuing_preserves_insertion_order() {
        let (mut controller, _rx) = setup();
        controller.queue(song("1", "a"));
        controller.queue(song("2", "b"));
        controller.queue(song("3", "c"));
        let q = controller.queue_store().read().cloned();
        assert_eq!(q.get(0).unwrap().read().1.id, "1");
        assert_eq!(q.get(1).unwrap().read().1.id, "2");
        assert_eq!(q.get(2).unwrap().read().1.id, "3");
    }
}
