use dioxus::prelude::*;

#[component]
pub fn NextIcon(filled: ReadSignal<bool>, size: Option<u8>) -> Element {
    let size = size.unwrap_or(32);
    if filled() {
        rsx! {
            svg {
                class: "icon",
                fill: "#000000",
                height: "{size}",
                view_box: "0 0 256 256",
                width: "{size}",
                xmlns: "http://www.w3.org/2000/svg",
                path { d: "M208,40V216a8,8,0,0,1-16,0V146.77L72.43,221.55A15.95,15.95,0,0,1,48,208.12V47.88A15.95,15.95,0,0,1,72.43,34.45L192,109.23V40a8,8,0,0,1,16,0Z" }
            }
        }
    } else {
        rsx! {
            svg {
                class: "icon",
                fill: "#000000",
                height: "{size}",
                view_box: "0 0 256 256",
                width: "{size}",
                xmlns: "http://www.w3.org/2000/svg",
                path { d: "M200,32a8,8,0,0,0-8,8v69.23L72.43,34.45A15.95,15.95,0,0,0,48,47.88V208.12a16,16,0,0,0,24.43,13.43L192,146.77V216a8,8,0,0,0,16,0V40A8,8,0,0,0,200,32ZM64,207.93V48.05l127.84,80Z" }
            }
        }
    }
}
