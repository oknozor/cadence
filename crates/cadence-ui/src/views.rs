use crate::navigation::navbar::Navbar;
use album::AlbumView;
use artist::ArtistView;
use dioxus::prelude::*;
use home::Home;
use library::LibraryView;
use now_playing::NowPlayingView;
use search::SearchView;
use settings::SettingsView;

mod album;
mod artist;
mod home;
mod library;
mod now_playing;
mod search;
mod settings;

#[derive(Debug, Clone, Routable, PartialEq, Eq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(WebNavbar)]
    #[route("/")]
    Home { },

    #[route("/now-playing")]
    NowPlayingView { },

    #[route("/search")]
    SearchView { },

    #[route("/library")]
    LibraryView { },

    #[route("/album/:id")]
    AlbumView { id: String },

    #[route("/artist/:id")]
    ArtistView { id: String },

    #[route("/settings")]
    SettingsView { },
}

#[component]
pub fn WebNavbar() -> Element {
    rsx! {
        div { class: "view", Outlet::<Route> {} }
        Navbar {}
    }
}
