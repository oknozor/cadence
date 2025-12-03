use crate::player::FullScreenPlayer;
use dioxus::prelude::*;

#[component]
pub fn NowPlayingView() -> Element {
    rsx! {
        FullScreenPlayer {}
    }
}
