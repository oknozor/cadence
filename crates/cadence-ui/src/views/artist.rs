use crate::components::{ArtistAbout, ArtistLiveEvents};
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

    let mut artist_name_signal = use_signal(String::new);

    use_effect(move || {
        if let Some(ref a) = artist() {
            if artist_name_signal() != a.name {
                tracing::info!("[ArtistView] Artist loaded: '{}', updating signal", a.name);
                artist_name_signal.set(a.name.clone());
            }
        }
    });

    let concerts = use_artist_concerts(artist_name_signal);

    use_effect(move || {
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
        let concerts_state = concerts();
        tracing::info!(
            "[ArtistView] Concerts resource state: {:?}",
            match &concerts_state {
                Some(Ok(list)) => format!("Ok({} concerts)", list.len()),
                Some(Err(e)) => format!("Err({})", e),
                None => "None (loading)".to_string(),
            }
        );

        rsx! {
            BackButton {}
            div { class: "artist-view",
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
                            ArtistLiveEvents { concerts: concert_list }
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
