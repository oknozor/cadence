use dioxus::prelude::*;

#[component]
pub fn AlbumCover(src: ReadSignal<String>) -> Element {
    rsx! {
        div { class: "album-cover",
            img { src: "{src}", alt: "Album cover" }
        }
    }
}
