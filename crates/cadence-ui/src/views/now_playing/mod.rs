use dioxus::prelude::*;
use components::FullScreenPlayer;

mod components;

#[component]
pub fn NowPlayingView() -> Element {
    rsx! {
        FullScreenPlayer {}
    }
}
