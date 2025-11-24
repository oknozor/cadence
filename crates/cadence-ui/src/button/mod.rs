use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    pub text: ReadSignal<String>,
    pub content_expanded: ReadSignal<Option<Element>>,

    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    rsx! {
        div {
            "expandable-button"
            button {
                class: "menu-button",
                ..props.attributes,
                div {
                    "{props.text}"
                }
            }
            if let Some(expanded_content) = props.content_expanded.read().cloned() {
                div {
                    class: "expanded-menu-button",
                    {expanded_content}
                }
            }
        }
    }
}
