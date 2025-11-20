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

    let cover = album
        .read()
        .as_ref()
        .and_then(|album| album.cover_art.clone())
        .unwrap_or_default();

    let cover_clone = cover.clone();
    let dominant_color = use_resource(move || {
        let cover = cover_clone.clone();
        async move {
            let script = format!(
                r#"
                const color = await window.getDominantColor("{cover}");
                dioxus.send(color);
                "#,
            );
            let mut eval = document::eval(&script);
            eval.recv::<String>().await.unwrap()
        }
    });

    rsx! {
        div {
            class: "album-view",
            match album() {
                Some(album) => rsx! {
                    div {
                        class: "album-info",
                        div {
                            class: "album-cover",
                            background_image: "url({cover})",
                            div {
                                class: "album-cover-overlay",
                                img {
                                    src: "{cover}",
                                    alt: "Album cover"
                                }
                            }
                        }
                        if let Some(year) = album.year {
                            h1 { "{album.name} ({year})" }
                        } else {
                            h1 { "{album.name}" }
                        }
                        h2 { "{album.artist}" }
                        match dominant_color.read_unchecked().as_ref() {
                            Some(v) => rsx! {
                                p { "{v}" }
                            },
                            _ => rsx! {
                                p { "hello" }
                            },
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
