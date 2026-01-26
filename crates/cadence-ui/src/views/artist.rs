use crate::{
    components::{AlbumList, ArtistGrid, BackButton, RoundedThumbnail, VerticalScroller},
    views::Route,
};
use cadence_core::hooks::{use_artist, use_artist_concerts};
use cadence_core::services::ticketmaster_client::Concert;
use dioxus::prelude::*;

#[component]
pub fn ArtistView(id: ReadSignal<String>) -> Element {
    let mut current_id = use_signal(|| id());
    let artist = use_artist(current_id);
    let mut scroll = use_signal(|| None);

    // Create a signal for artist name that updates when artist changes
    // This must be at the top level, not inside conditional blocks
    let mut artist_name_signal = use_signal(String::new);

    // Update artist name when artist data is available
    use_effect(move || {
        if let Some(ref a) = artist() {
            if artist_name_signal() != a.name {
                tracing::info!("[ArtistView] Artist loaded: '{}', updating signal", a.name);
                artist_name_signal.set(a.name.clone());
            }
        }
    });

    // Call the concerts hook unconditionally at the top level
    let concerts = use_artist_concerts(artist_name_signal);

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
        // Log concert resource state
        let concerts_state = concerts();
        tracing::info!("[ArtistView] Concerts resource state: {:?}",
            match &concerts_state {
                Some(Ok(list)) => format!("Ok({} concerts)", list.len()),
                Some(Err(e)) => format!("Err({})", e),
                None => "None (loading)".to_string(),
            }
        );

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

                    // Concert section
                    if let Some(Ok(concert_list)) = concerts_state {
                        if !concert_list.is_empty() {
                            ArtistConcerts { concerts: concert_list }
                        }
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
        rsx! {
            div { class: "loading" }
        }
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

#[component]
pub fn ArtistConcerts(concerts: Vec<Concert>) -> Element {
    rsx! {
        section { class: "artist-section artist-concerts",
            h2 { "Upcoming Concerts" }
            div { class: "concerts-list",
                for concert in concerts {
                    a {
                        class: "concert-item",
                        href: "{concert.url}",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        div { class: "concert-date", "{concert.date}" }
                        div { class: "concert-details",
                            div { class: "concert-name", "{concert.name}" }
                            div { class: "concert-venue", "{concert.venue}" }
                            div { class: "concert-location", "{concert.city}, {concert.country}" }
                        }
                        div { class: "concert-tickets", "Get Tickets →" }
                    }
                }
            }
        }
    }
}
