use cadence_core::{
    model::Song,
    state::{CONTROLLER, ControllerExt, ControllerStoreExt},
};
use dioxus::prelude::*;

use crate::components::{DotIcon, ItemInfo, ItemRow, PlusIcon, Thumbnail};

#[component]
pub fn SongItemInfo(song: Song, thumbnail_size: Option<u32>) -> Element {
    let mut controller = CONTROLLER.resolve();
    let size = thumbnail_size.unwrap_or(50);
    let id = song.id.clone();
    let is_active = use_memo(move || {
        let current = controller.current_song_id()();
        current == Some(id.clone())
    });
    let is_paused = !*controller.is_playing().read();

    let Song {
        title,
        artist,
        cover_art,
        ..
    } = song.clone();

    let thumbnail = cover_art.map(|src| {
        rsx! {
            Thumbnail { size, name: title.clone(), src }
        }
    });

    let content = rsx! {
        ItemInfo {
            primary: title,
            secondary: "Song · {artist}",
            is_active,
            is_paused,
        }
    };

    let action = Some(rsx! {
        DotIcon { size: 18 }
        PlusIcon { size: 18, filled: false }
    });

    let callback = move || controller.play_now(song.clone());

    rsx! {
        ItemRow {
            thumbnail,
            content,
            action,
            callback,
        }
    }
}
