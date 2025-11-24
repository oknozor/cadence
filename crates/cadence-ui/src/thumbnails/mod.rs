use dioxus::prelude::*;

#[component]
pub fn RoundedThumbnail(size: u32, name: String, src: String) -> Element {
    rsx! {
        img {
            class: "thumbnail-rounded",
            src: "{src}",
            alt: "{name}",
            width: "{size}px",
            height: "{size}px",
        }
    }
}

#[component]
pub fn Thumbnail(size: u32, name: String, src: String) -> Element {
    rsx! {
        img {
            class: "thumbnail",
            src: "{src}",
            alt: "{name}",
            width: "{size}px",
            height: "{size}px",
        }
    }
}
