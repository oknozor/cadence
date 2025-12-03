use crate::navigation::navbar::Navbar;
use album::AlbumView;
use dioxus::prelude::*;
use home::Home;
use library::LibraryView;
use now_playing::NowPlayingView;
use search::SearchView;

mod album;
mod home;
mod library;
mod search;
mod now_playing;

#[derive(Debug, Clone, Routable, PartialEq)]
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
}

#[component]
pub fn WebNavbar() -> Element {
    rsx! {
        div { class: "view", Outlet::<Route> {} }
        Navbar {}
    }
}
