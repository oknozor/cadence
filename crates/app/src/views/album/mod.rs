use cadence_core::services::subsonic_client::SUBSONIC_CLIENT;
use cadence_ui::{
    album::{AlbumActionBar, AlbumCover, AlbumTitle},
    track::TrackList,
};
use dioxus::prelude::*;

#[component]
pub fn AlbumView(id: String) -> Element {
    let album = use_resource(move || {
        let id = id.clone();
        async move {
            SUBSONIC_CLIENT()
                .clone()
                .unwrap()
                .get_album(&id)
                .await
                .unwrap()
        }
    });

    let cover = album
        .read()
        .as_ref()
        .and_then(|album| album.cover_art.clone())
        .unwrap_or_default();

    rsx! {
        div { class: "album-view",
            match album() {
                Some(album) => rsx! {
                    AlbumCover { src: cover }
                    div { class: "album-info view",
                        AlbumTitle {
                            name: album.name.clone(),
                            artist: album.artist.clone(),
                            year: album.year.clone(),
                        }
                        // TODO: songs should probably be behind a singal ?
                        AlbumActionBar { songs: album.songs.clone() }
                        TrackList { album }
                    }
                },
                None => rsx! {
                    div { class: "loading", "Loading album..." }
                },
            }
        }
    }
}
