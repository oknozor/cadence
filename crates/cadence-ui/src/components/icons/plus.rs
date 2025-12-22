use dioxus::prelude::*;

#[component]
pub fn PlusIcon(size: Option<u32>, filled: ReadSignal<bool>) -> Element {
    let size = size.unwrap_or(32);

    rsx! {
        if filled() {
            svg {
                class: "icon",
                fill: "#2ec27e",
                height: "{size}",
                view_box: "0 0 256 256",
                width: "{size}",
                xmlns: "http://www.w3.org/2000/svg",
                path { d: "M128,24A104,104,0,1,0,232,128,104.11,104.11,0,0,0,128,24Zm45.66,85.66-56,56a8,8,0,0,1-11.32,0l-24-24a8,8,0,0,1,11.32-11.32L112,148.69l50.34-50.35a8,8,0,0,1,11.32,11.32Z" }
            }
        } else {
            svg {
                class: "icon",
                fill: "#000000",
                height: "{size}",
                view_box: "0 0 256 256",
                width: "{size}",
                xmlns: "http://www.w3.org/2000/svg",
                path { d: "M128,24A104,104,0,1,0,232,128,104.11,104.11,0,0,0,128,24Zm0,192a88,88,0,1,1,88-88A88.1,88.1,0,0,1,128,216Zm48-88a8,8,0,0,1-8,8H136v32a8,8,0,0,1-16,0V136H88a8,8,0,0,1,0-16h32V88a8,8,0,0,1,16,0v32h32A8,8,0,0,1,176,128Z" }
            }
        }
    }
}
