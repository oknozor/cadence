use dioxus::prelude::*;

#[component]
pub fn AlbumTitle(name: String, artist: String, year: Option<i64>) -> Element {
    rsx! {
        if let Some(year) = year {
            h1 { class: "album-title", "{name} ({year})" }
        } else {
            h1 { class: "album-title", "{name}" }
        }
        h2 { class: "album-artist", "{artist}" }
    }
}
