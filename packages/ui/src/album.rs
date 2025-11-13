use crate::client::SUBSONIC_CLIENT;
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

    rsx! {
        div {
            class: "album-view",
            match album() {
                Some(album) => rsx! {
                    div {
                        class: "album-info",
                        h1 { "{album.name}" }
                        p { "Artist: {album.artist}" }
                        img {
                            src: "{album.cover_art.as_ref().unwrap_or(&String::new())}",
                            alt: "Album cover"
                        }
                    }

                    div {
                        class: "track-list",
                        for track in album.songs {
                           crate::track::Track { title: track.title.clone(), track_id: track.id.clone() }
                        }
                    }
                },
                None => rsx! {
                    div {
                        class: "loading",
                        "Loading album..."
                    }
                },
            }
        }
    }
}
