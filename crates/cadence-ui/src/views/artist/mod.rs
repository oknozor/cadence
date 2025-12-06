use crate::{album::AlbumList, scroller::VerticalScroller, views::Route};
use cadence_core::hooks::use_artist;
use dioxus::prelude::*;

#[component]
pub fn ArtistView(id: String) -> Element {
    let artist = use_artist(id);

    // Elder Scrobz chart: render only if a Scrobz account is present
    // TODO: Wire to a real setting/state; using a temporary flag for now
    let has_scrobz_account = false;
    let nav = navigator();

    if let Some(artist) = artist() {
        rsx! {
            div { class: "artist-info",
                div { class: "artist-header",
                    if let Some(img) = artist.cover_art.as_ref() {
                        img {
                            class: "artist-image",
                            src: "{img}",
                            alt: "{artist.name}",
                            width: "200px",
                            height: "200px"
                        }
                    } else {
                        div { class: "artist-image placeholder", "🎤" }
                    }

                    div { class: "artist-title",
                        h1 { class: "artist-name", "{artist.name}" }
                    }

                    div { class: "artist-actions",
                        button { class: "btn primary", onclick: move |_| {}, "Play" }
                        button { class: "btn", onclick: move |_| {}, "Shuffle" }
                    }
                }

                VerticalScroller { 
                    section { class: "artist-section",
                        h2 { "About" }
                        if let Some(bio) = artist.bio {
                            div {
                                class: "wikidata-summary",
                                dangerous_inner_html: bio
                            }
                        }
                    }

                    section { class: "artist-section",
                        AlbumList {
                            title: "Albums".to_string(),
                            albums: artist.albums,
                            on_album_select: move |album_id| {
                                nav.push(Route::AlbumView { id: album_id });
                            }
                        }
                    }

                    // Elder Scrobz chart (conditional)
                    if has_scrobz_account {
                        section { class: "artist-section",
                            h2 { "Top Tracks (Scrobz)" }
                            // TODO: Replace with a real chart/list; mocked for now
                            ul { class: "scrobz-chart",
                                li { "1. Mock Track 1" }
                                li { "2. Mock Track 2" }
                                li { "3. Mock Track 3" }
                            }
                        }
                    }

                    // Collabs
                    section { class: "artist-section",
                        h2 { "More like this" }
                        if artist.similar.is_empty() {
                        } else {
                            ul { class: "collabs",
                                for similar in artist.similar.into_iter() {
                                    // TODO
                                    li { "{similar.name}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        rsx! { "Loading" }
    }
}
