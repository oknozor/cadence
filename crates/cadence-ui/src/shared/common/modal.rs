use dioxus::prelude::*;

const DRAG_CLOSE_THRESHOLD: f64 = 300.0;

#[component]
pub fn MenuModal(open: WriteSignal<bool>, children: Element) -> Element {
    let mut dragging = use_signal(|| false);
    let mut start_y = use_signal(|| 0.0);
    let mut delta = use_signal(|| 0.0);

    rsx! {
        div {
            class: if open() { "bottom-modal opened" } else { "bottom-modal" },
            transform: if dragging() {
                "translateY({delta}px)"
            } else if open() {
                "translateY(0)"
            } else {
                "translateY(100%)"
            },
            transition: "transform 0.3s cubic-bezier(.23,1.04,.41,.99)",
            ontouchstart: move |e| {
                let touch_y = e.touches()[0].page_coordinates().y;
                start_y.set(touch_y);
                dragging.set(true);
            },
            ontouchmove: move |e| {
                let current_y = e.touches()[0].page_coordinates().y;
                let d = current_y - start_y();
                if d > 0.0 {
                    delta.set(d);
                }
            },
            ontouchend: move |_| {
                dragging.set(false);
                if delta() > DRAG_CLOSE_THRESHOLD {
                    open.toggle()
                } else {
                    delta.set(0.0);
                }
            },
            onmousedown: move |e| {
                let y = e.page_coordinates().y;
                start_y.set(y);
                dragging.set(true);
            },
            onmousemove: move |e| {
                if dragging() {
                    let y = e.page_coordinates().y;
                    let d = y - start_y();
                    if d > 0.0 {
                        delta.set(d);
                    }
                }
            },
            onmouseup: move |_| {
                if dragging() {
                    dragging.set(false);
                    if delta() > DRAG_CLOSE_THRESHOLD {
                        open.toggle()
                    } else {
                        delta.set(0.0);
                    }
                }
            },

            div { class: "grabber" }
            div { class: "modal-content", {children} }
        }
    }
}
