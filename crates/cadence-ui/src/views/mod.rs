use crate::navigation::navbar::Navbar;
use album::AlbumView;
use dioxus::prelude::*;
use home::Home;
use library::LibraryView;
use search::SearchView;

mod album;
mod home;
mod library;
mod search;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(WebNavbar)]
    #[route("/")]
    Home { },
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
        Outlet::<Route> {}
        Navbar {}
    }
}
