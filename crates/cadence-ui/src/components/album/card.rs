use crate::components::ItemInfo;
use cadence_core::model::Album;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct AlbumCardProps {
    pub album: Album,
    #[props(default = |_| {})]
    pub on_press: EventHandler<Album>,
    #[props(default = |_| {})]
    pub on_click: EventHandler<String>,
}

#[component]
pub fn AlbumCard(props: AlbumCardProps) -> Element {
    let album_clone = props.album.clone();
    rsx! {
        div {
            class: "album-card",
            key: "{props.album.id}",
            onpointerdown: move |_| {
                props.on_press.call(album_clone.clone());
            },
            onpointerup: move |_| {
                props.on_click.call(props.album.id.clone());
            },
            if let Some(cover) = props.album.cover_art.as_ref() {
                img {
                    src: "{cover}",
                    alt: "{props.album.name}",
                    width: "100px",
                    height: "100px",
                }
            } else {
                div { class: "no-cover", "🎵" }
            }

            ItemInfo {
                primary: props.album.name,
                secondary: props.album.artist,
                is_active: false,
                is_paused: false,
            }
        }
    }
}
