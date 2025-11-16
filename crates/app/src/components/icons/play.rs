use dioxus::prelude::*;

#[component]
pub fn PlayIcon(is_playing: ReadSignal<bool>, size: u8) -> Element {
    rsx! {
        if is_playing() {
            svg {
                class: "icon",
                fill: "#000000",
                height: "32",
                view_box: "0 0 256 256",
                width: "32",
                xmlns: "http://www.w3.org/2000/svg",
                path { d: "M128,26A102,102,0,1,0,230,128,102.12,102.12,0,0,0,128,26Zm0,192a90,90,0,1,1,90-90A90.1,90.1,0,0,1,128,218ZM110,96v64a6,6,0,0,1-12,0V96a6,6,0,0,1,12,0Zm48,0v64a6,6,0,0,1-12,0V96a6,6,0,0,1,12,0Z" }
            }
        } else {
            svg {
                class: "icon",
                fill: "#000000",
                height: "32",
                view_box: "0 0 256 256",
                width: "32",
                xmlns: "http://www.w3.org/2000/svg",
                path { d: "M128,26A102,102,0,1,0,230,128,102.12,102.12,0,0,0,128,26Zm0,192a90,90,0,1,1,90-90A90.1,90.1,0,0,1,128,218Zm47.18-95.09-64-40A6,6,0,0,0,102,88v80a6,6,0,0,0,9.18,5.09l64-40a6,6,0,0,0,0-10.18ZM114,157.17V98.83L160.68,128Z" }
            }
        }
    }
}
