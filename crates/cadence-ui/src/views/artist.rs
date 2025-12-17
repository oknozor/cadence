use crate::{
    components::{AlbumList, ArtistGrid, BackButton, RoundedThumbnail, VerticalScroller},
    views::Route,
};
use cadence_core::hooks::use_artist;
use dioxus::prelude::*;

#[component]
pub fn ArtistView(id: ReadSignal<String>) -> Element {
    let mut current_id = use_signal(|| id());
    let artist = use_artist(current_id);
    let mut scroll = use_signal(|| None);
    use_effect(move || {
        // reading the current id, so it becomes a dependency of the effect
        let _ = current_id.read();
        scroll.set(Some(0.0));
    });

    let nav = navigator();

    let on_artist_card_clicked = move |artist_id| {
        current_id.set(artist_id);
    };

    let on_album_card_clicked = move |album_id| {
        nav.push(Route::AlbumView { id: album_id });
    };

    if let Some(artist) = artist() {
        rsx! {
            BackButton {}
            div { class: "artist-info",
                VerticalScroller { scroll,
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
                            on_click: on_album_card_clicked,
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
