use crate::shared::{AlbumList, ArtistGrid, RoundedThumbnail};
use crate::{shared::VerticalScroller, views::Route};
use cadence_core::hooks::use_artist;
use dioxus::prelude::*;

mod components;

#[component]
pub fn ArtistView(id: String) -> Element {
    let artist = use_artist(id);
    let nav = navigator();

    let on_artist_card_clicked = move |artist_id| {
        if let Some(a) = nav.push(Route::ArtistView { id: artist_id }) {
            tracing::debug!("Clicked on artist card with ID: {:?}", a);
        }
    };

    let on_album_card_clicked = move |album_id| {
        nav.push(Route::AlbumView { id: album_id });
    };

    if let Some(artist) = artist() {
        rsx! {
            div { class: "artist-info",
                VerticalScroller {
                    div { class: "artist-header",
                        if let Some(src) = artist.cover_art {
                            RoundedThumbnail {
                                size: 128,
                                src,
                                name: artist.name.clone(),
                            }
                        }

                        div { class: "artist-title",
                            h1 { class: "artist-name", "{artist.name}" }
                        }
                    }

                    if let Some(bio) = artist.bio {
                        ArtistAbout { bio }
                    }

                    section { class: "artist-section",
                        AlbumList {
                            title: "Albums".to_string(),
                            albums: artist.albums,
                            on_card_clicked: on_album_card_clicked,
                        }
                    }

                    if !artist.similar.is_empty() {
                        h2 { "More like this" }
                        ArtistGrid {
                            artists: artist.similar,
                            on_card_clicked: on_artist_card_clicked,
                        }
                    }
                }
            }
        }
    } else {
        rsx! { "Loading" }
    }
}

#[component]
pub fn ArtistAbout(bio: String) -> Element {
    rsx! {
        section { class: "artist-section",
            h2 { "About" }
            div { class: "wikidata-summary", dangerous_inner_html: bio }
        }
    }
}
