use dioxus::prelude::*;

#[component]
pub fn PreviousIcon(filled: ReadSignal<bool>, size: Option<u8>) -> Element {
    let size = size.unwrap_or(32);
    if filled() {
        rsx! {
            svg {
                class: "icon",
                fill: "#000000",
                height: "{size}",
                view_box: "0 0 256 256",
                width: "32",
                xmlns: "http://www.w3.org/2000/svg",
                path { d: "M208,47.88V208.12a16,16,0,0,1-24.43,13.43L64,146.77V216a8,8,0,0,1-16,0V40a8,8,0,0,1,16,0v69.23L183.57,34.45A15.95,15.95,0,0,1,208,47.88Z" }
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
                path { d: "M199.81,34a16,16,0,0,0-16.24.43L64,109.23V40a8,8,0,0,0-16,0V216a8,8,0,0,0,16,0V146.77l119.57,74.78A15.95,15.95,0,0,0,208,208.12V47.88A15.86,15.86,0,0,0,199.81,34ZM192,208,64.16,128,192,48.07Z" }
            }
        }
    }
}
