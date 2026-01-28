use dioxus::prelude::*;
use std::path::absolute;

#[derive(Props, Clone, PartialEq)]
pub struct AlbumCoverProps {
    src: ReadSignal<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn AlbumCover(props: AlbumCoverProps) -> Element {
    rsx! {
        div { class: "album-cover", ..props.attributes,
            img { src: "{props.src}", alt: "Album cover" }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct AlbumCoverBackgroundProps {
    src: ReadSignal<String>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    pub children: Element,
}

#[component]
pub fn AlbumCoverBackground(props: AlbumCoverBackgroundProps) -> Element {
    rsx! {
        div {
            class: "album-cover-background",
            background_image: r#"linear-gradient(to bottom, rgba(0,0,0,0) 60%, rgba(0,0,0,1) 100%), url('{props.src}')"#,
            ..props.attributes,
            {props.children}
        }
    }
}

