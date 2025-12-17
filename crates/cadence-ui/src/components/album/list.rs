use cadence_core::model::Album;
use dioxus::prelude::*;

use super::card::AlbumCard;
use crate::components::HorizontalScroller;

#[derive(Props, Clone, PartialEq)]
pub struct AlbumListProps {
    pub title: String,
    pub albums: Vec<Album>,
    pub on_click: EventHandler<String>,
    #[props(default = |_| {})]
    pub on_press: EventHandler<Album>,
}

#[component]
pub fn AlbumList(props: AlbumListProps) -> Element {
    rsx! {
        div { class: "tracklist-container",
            h2 { "{props.title}" }

            HorizontalScroller {
                for album in props.albums {
                    AlbumCard { album, on_click: props.on_click, on_press: props.on_press,  }
                }
            }
        }
    }
}
