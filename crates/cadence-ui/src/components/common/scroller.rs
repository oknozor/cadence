use std::rc::Rc;

use dioxus::prelude::*;

#[component]
pub fn HorizontalScroller(children: Element) -> Element {
    rsx! {
        div { class: "horizontal-scroller", {children} }
    }
}

#[component]
pub fn VerticalScroller(children: Element, scroll: ReadSignal<Option<f64>>) -> Element {
    let mut data: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    use_effect(move || {
        if let Some(scroll) = scroll() {
            let data = data().expect("No vertical scroll element found");
            spawn(async move {
                data.scroll((scroll, scroll).into(), ScrollBehavior::Instant)
                    .await
                    .unwrap();
            });
        }
    });

    rsx! {
        div {
            onmounted:move |event| {
                data.set(Some(event.data));
            },
            class: "vertical-scroller",
            {children}
        }
    }
}
