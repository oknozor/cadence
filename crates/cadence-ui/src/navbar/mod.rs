use dioxus::prelude::*;
use std::time::Duration;

#[component]
pub fn NavbarItem(
    label: String,
    active: Signal<bool>,
    onclick: EventHandler<()>,
    children: Element,
) -> Element {
    let mut animate = use_signal(|| false);
    rsx! {
        div {
            class: "navbar-item",
            onclick: move |_| {
                active.set(!active());
                animate.set(true);
                spawn(async move {
                    dioxus_sdk::time::sleep(Duration::from_millis(200)).await;
                    animate.set(false);
                });

                onclick.call(())
            },
            div { class: if animate() { "navbar-item-icon active" } else { "navbar-item-icon" },
                {children}
            }
            div { class: "navbar-item-label", "{label}" }
        }
    }
}
