use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ProgressProps {
    pub value: ReadSignal<Option<f64>>,
    #[props(default = ReadSignal::new(Signal::new(100.0)))]
    pub max: ReadSignal<f64>,
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    pub children: Element,
}

#[component]
pub fn Progress(props: ProgressProps) -> Element {
    let percentage = use_memo(move || {
        props.value.cloned().map(|v| {
            let max = (props.max)();
            (v / max) * 100.0
        })
    });

    let state = use_memo(move || match percentage() {
        Some(_) => "loading",
        None => "indeterminate",
    });

    rsx! {
        div {
            role: "progressbar",
            "aria-valuemin": 0,
            "aria-valuemax": props.max,
            "aria-valuenow": props.value.cloned(),
            "data-state": state,
            "data-value": props.value.cloned().map(|v| v.to_string()),
            "data-max": props.max,
            style: percentage().map(|p| format!("--progress-value: {p}%")),
            ..props.attributes,

            {props.children}
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ProgressIndicatorProps {
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,
    pub children: Element,
}

#[component]
pub fn ProgressIndicator(props: ProgressIndicatorProps) -> Element {
    rsx! {
        div { ..props.attributes, {props.children} }
    }
}
