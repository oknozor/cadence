use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ExpandableButtonProps {
    pub text: ReadSignal<String>,
    pub text_expanded: ReadSignal<String>,
    pub active: Signal<bool>,
    pub inner_active: Signal<bool>,
    pub onclick: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    pub onclick: Option<EventHandler<MouseEvent>>,
    pub active: Signal<bool>,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    pub children: Element,
}

#[component]
pub fn MenuButton(mut props: ButtonProps) -> Element {
    let active = *props.active.read();
    rsx! {
        button {
            class: if active { "menu-button active" } else { "menu-button" },
            onclick: move |evt| {
                if let Some(onclick) = props.onclick {
                    *props.active.write() = !active;
                    onclick.call(evt);
                }
            },
            ..props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn ExpandableButton(mut props: ExpandableButtonProps) -> Element {
    let active = *props.active.read();
    let inner_active = *props.inner_active.read();

    rsx! {
        div { class: if active && inner_active { "expandable-wrapper expanded active" } else if active { "expandable-wrapper expanded" } else { "expandable-wrapper" },
            MenuButton {
                active: props.active,
                onclick: move |evt| {
                    if let Some(onclick) = props.onclick {
                        *props.active.write() = !active;
                        if !*props.active.read() {
                            *props.inner_active.write() = false;
                        }
                        onclick.call(evt);
                    }
                },
                "{props.text}"
            }
            div { class: if active && inner_active { "expanded-content expanded active" } else if active { "expanded-content expanded" } else { "expanded-content" },
                MenuButton {
                    id: "inner",
                    active: props.inner_active,
                    onclick: move |_| {},
                    "{props.text_expanded}"
                }
            }
        }
    }
}
