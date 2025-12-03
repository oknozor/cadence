use dioxus::prelude::*;

#[component]
pub fn PlayIcon(is_playing: ReadSignal<bool>, size: Option<u8>) -> Element {
    let size = size.unwrap_or(32);
    rsx! {
        if is_playing() {
            svg {
                class: "icon",
                fill: "#000000",
                height: "{size}",
                view_box: "0 0 256 256",
                width: "{size}",
                xmlns: "http://www.w3.org/2000/svg",
                path { d: "M216,48V208a16,16,0,0,1-16,16H160a16,16,0,0,1-16-16V48a16,16,0,0,1,16-16h40A16,16,0,0,1,216,48ZM96,32H56A16,16,0,0,0,40,48V208a16,16,0,0,0,16,16H96a16,16,0,0,0,16-16V48A16,16,0,0,0,96,32Z" }
            }
        } else {
            svg {
                class: "icon",
                fill: "#000000",
                height: "{size}",
                view_box: "0 0 256 256",
                width: "{size}",
                xmlns: "http://www.w3.org/2000/svg",
                path { d: "M240,128a15.74,15.74,0,0,1-7.6,13.51L88.32,229.65a16,16,0,0,1-16.2.3A15.86,15.86,0,0,1,64,216.13V39.87a15.86,15.86,0,0,1,8.12-13.82,16,16,0,0,1,16.2.3L232.4,114.49A15.74,15.74,0,0,1,240,128Z" }
            }
        }
    }
}

#[component]
pub fn PlayIconCircle(is_playing: ReadSignal<bool>, size: Option<u8>) -> Element {
    let size = size.unwrap_or(32);
    rsx! {
        if is_playing() {
            svg {
                class: "icon",
                fill: "#000000",
                height: "{size}",
                view_box: "0 0 256 256",
                width: "{size}",
                xmlns: "http://www.w3.org/2000/svg",
                path { d: "M128,26A102,102,0,1,0,230,128,102.12,102.12,0,0,0,128,26Zm0,192a90,90,0,1,1,90-90A90.1,90.1,0,0,1,128,218ZM110,96v64a6,6,0,0,1-12,0V96a6,6,0,0,1,12,0Zm48,0v64a6,6,0,0,1-12,0V96a6,6,0,0,1,12,0Z" }
            }
        } else {
            svg {
                class: "icon",
                fill: "#000000",
                height: "{size}",
                view_box: "0 0 256 256",
                width: "{size}",
                xmlns: "http://www.w3.org/2000/svg",
                path { d: "M128,26A102,102,0,1,0,230,128,102.12,102.12,0,0,0,128,26Zm0,192a90,90,0,1,1,90-90A90.1,90.1,0,0,1,128,218Zm47.18-95.09-64-40A6,6,0,0,0,102,88v80a6,6,0,0,0,9.18,5.09l64-40a6,6,0,0,0,0-10.18ZM114,157.17V98.83L160.68,128Z" }
            }
        }
    }
}
