use components::FullScreenPlayer;
use dioxus::prelude::*;

mod components;

#[component]
pub fn NowPlayingView() -> Element {
    rsx! {
        FullScreenPlayer {}
    }
}
