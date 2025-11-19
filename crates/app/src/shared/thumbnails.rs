use dioxus::prelude::*;

#[component]
pub fn RoundedThumbnail(size: u32, artist: String, src: String) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        img {
            class: "thumbnail-rounded",
            src: "{src}",
            alt: "{artist}",
            width: "{size}px",
            height: "{size}px"
        }
    }
}

#[component]
pub fn Thumbnail(size: u32, artist: String, src: String) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        img {
            class: "thumbnail",
            src: "{src}",
            alt: "{artist}",
            width: "{size}px",
            height: "{size}px"
        }
    }
}
