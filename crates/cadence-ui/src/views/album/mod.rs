use crate::components::{AlbumActionBar, AlbumCover, TrackList, TrackMenuModal, VerticalScroller};
use cadence_core::hooks::use_album;
use components::{AlbumMenuModal, AlbumTitle};
use dioxus::prelude::*;

mod components;

#[component]
pub fn AlbumView(id: ReadSignal<String>) -> Element {
    let current_id = use_signal(|| id());
    let album = use_album(current_id);
    let album_modal_open = use_signal(|| false);
    let mut song_modal_open = use_signal(|| false);
    let mut song_selected = use_signal(|| None);

    let action_clicked = move |song| {
        debug!("Clicked on song: {:?}", song);
        song_selected.set(Some(song));
        song_modal_open.toggle();
    };

    rsx! {
        match album() {
            Some(album) => rsx! {
                div { class: "album-info",
                    AlbumCover { src: album.cover_art.clone().unwrap_or_default(), width: "200px" }
                    AlbumTitle {
                        name: album.name.clone(),
                        artist: album.artist.clone(),
                        year: album.year,
                    }
                    AlbumActionBar { songs: album.songs.clone(), modal_open: album_modal_open }
                    VerticalScroller {
                        TrackList { album: album.clone(), action_clicked }
                    }
                    AlbumMenuModal { open: album_modal_open, album }
                    if let Some(song) = song_selected() {
                        TrackMenuModal { open: song_modal_open, song }
                    }
                }
            },
            None => rsx! {
                div { class: "loading", "Loading album..." }
            },
        }
    }
}
