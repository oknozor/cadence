use dioxus::prelude::*;

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
