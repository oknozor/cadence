use dioxus::prelude::*;

use crate::queue::Queue;

#[component]
pub fn NowPlayingView() -> Element {
    rsx! {
        Queue {  }
    }
}
