use crate::services::subsonic_client::SUBSONIC_CLIENT;
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
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
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
                           crate::components::track::Track { track: track }
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
