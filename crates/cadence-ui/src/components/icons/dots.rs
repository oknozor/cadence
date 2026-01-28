use dioxus::prelude::*;

#[component]
pub fn DotIcon(size: Option<u8>) -> Element {
    let size = size.unwrap_or(32);

    rsx! {
        svg {
            class: "icon",
            fill: "#000000",
            height: "{size}",
            view_box: "0 0 256 256",
            width: "{size}",
            xmlns: "http://www.w3.org/2000/svg",
            path { d: "M140,128a12,12,0,1,1-12-12A12,12,0,0,1,140,128ZM128,72a12,12,0,1,0-12-12A12,12,0,0,0,128,72Zm0,112a12,12,0,1,0,12,12A12,12,0,0,0,128,184Z" }
        }
    }
}
