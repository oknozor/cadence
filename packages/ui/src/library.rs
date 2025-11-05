//! Music library browser component

use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Album {
    pub id: String,
    pub name: String,
    pub artist: String,
    pub year: Option<u16>,
    pub cover_art: Option<String>,
}

#[component]
pub fn LibraryBrowser(albums: Vec<Album>, on_album_select: EventHandler<String>) -> Element {
    rsx! {
        div {
            class: "library-container",
            h2 { "Music Library" }

            div {
                class: "albums-grid",
                for album in albums {
                    div {
                        class: "album-card",
                        key: "{album.id}",
                        onclick: move |_| on_album_select.call(album.id.clone()),

                        if let Some(cover) = &album.cover_art {
                            img {
                                src: "{cover}",
                                alt: "{album.name}",
                            }
                        } else {
                            div {
                                class: "no-cover",
                                "🎵"
                            }
                        }

                        div {
                            class: "album-info",
                            h3 { "{album.name}" }
                            p { "{album.artist}" }
                            if let Some(year) = album.year {
                                p { class: "year", "{year}" }
                            }
                        }
                    }
                }
            }
        }
    }
}
