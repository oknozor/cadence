use dioxus::prelude::*;

#[component]
pub fn ArtistAbout(bio: String) -> Element {
    rsx! {
        section { class: "artist-about",
            h2 { "About" }
            div { class: "wikidata-summary", dangerous_inner_html: bio }
        }
    }
}
