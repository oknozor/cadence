You are an expert assistant for this project using [Dioxus 0.7](https://dioxuslabs.com/learn/0.7). Dioxus 0.7 changes
every API in Dioxus. Only use the 0.7 documentation. `cx`, `Scope`, and `use_state` are gone.

Project-wide rules

- Do not add comments inside generated code. This applies to Rust, SCSS, CSS, JS/TS, and RSX. Use self-descriptive names
  and structure; place explanations in prose outside of code blocks if needed.
- Provide concise code examples with detailed descriptions. Prefer minimal, focused snippets.
- Match the existing code style in this repo: naming, imports, formatting, and file layout.

# Dioxus Dependency

You can add Dioxus to your `Cargo.toml` like this:

```toml
[dependencies]
dioxus = { version = "0.7.0" }

[features]
default = ["web", "webview", "server"]
web = ["dioxus/web"]
webview = ["dioxus/desktop"]
server = ["dioxus/server"]
```

# Launching your application

You need to create a main function that sets up the Dioxus runtime and mounts your root component.

```rust
use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! { "Hello, Dioxus!" }
}
```

Then serve with `dx serve`:

```sh
curl -sSL http://dioxus.dev/install.sh | sh
dx serve
```

# UI with RSX

```rust
rsx! {
    div {
        class: "container",
        color: "red",
        width: if condition { "100%" },
        "Hello, Dioxus!"
    }
    for i in 0..5 {
        div { "{i}" }
    }
    if condition {
        div { "Condition is true!" }
    }

    {children}
    {(0..5).map(|i| rsx! { span { "Item {i}" } })}
}
```

# Assets

The asset macro can be used to link to local files to use in your project. All links start with `/` and are relative to
the root of your project.

```rust
rsx! {
    img { src: asset!("/assets/image.png"), alt: "An image" }
}
```

## Styles

The `document::Stylesheet` component will inject the stylesheet into the `<head>` of the document

```rust
rsx! {
    document::Stylesheet { href: asset!("/assets/styles.css") }
}
```

# Components

Components are the building blocks of apps

* Component are functions annotated with the `#[component]` macro.
* The function name must start with a capital letter or contain an underscore.
* A component re-renders only under two conditions:
    1. Its props change (as determined by `PartialEq`).
    2. An internal reactive state it depends on is updated.

```rust
#[component]
fn Input(mut value: Signal<String>) -> Element {
    rsx! {
        input {
            value,
            oninput: move |e| { *value.write() = e.value(); },
            onkeydown: move |e| {
                if e.key() == Key::Enter { value.write().clear(); }
            },
        }
    }
}
```

Each component accepts function arguments (props)

* Props must be owned values, not references. Use `String` and `Vec<T>` instead of `&str` or `&[T]`.
* Props must implement `PartialEq` and `Clone`.
* To make props reactive and copy, you can wrap the type in `ReadOnlySignal`. Any reactive state like memos and
  resources that read `ReadOnlySignal` props will automatically re-run when the prop changes.

# State

A signal is a wrapper around a value that automatically tracks where it's read and written. Changing a signal's value
causes code that relies on the signal to rerun.

## Local State

The `use_signal` hook creates state that is local to a single component. You can call the signal like a function (e.g.
`my_signal()`) to clone the value, or use `.read()` to get a reference. `.write()` gets a mutable reference to the
value.

Use `use_memo` to create a memoized value that recalculates when its dependencies change. Memos are useful for expensive
calculations that you don't want to repeat unnecessarily.

```rust
#[component]
fn Counter() -> Element {
    let mut count = use_signal(|| 0);
    let mut doubled = use_memo(move || count() * 2); // doubled will re-run when count changes because it reads the signal

    rsx! {
		h1 { "Count: {count}" } // Counter will re-render when count changes because it reads the signal
		h2 { "Doubled: {doubled}" }
		button {
			onclick: move |_| *count.write() += 1, // Writing to the signal rerenders Counter
			"Increment"
		}
		button {
			onclick: move |_| count.with_mut(|count| *count += 1), // use with_mut to mutate the signal
			"Increment with with_mut"
		}
	}
}
```

## Context API

The Context API allows you to share state down the component tree. A parent provides the state using
`use_context_provider`, and any child can access it with `use_context`

```rust
#[component]
fn App() -> Element {
    let mut theme = use_signal(|| "light".to_string());
    use_context_provider(|| theme);
    rsx! { Child {} }
}

#[component]
fn Child() -> Element {
    let theme = use_context::<Signal<String>>();
    rsx! { div { "Current theme: {theme}" } }
}
```

# Async

For state that depends on an asynchronous operation (like a network request), Dioxus provides a hook called
`use_resource`. This hook manages the lifecycle of the async task and provides the result to your component.

* The `use_resource` hook takes an `async` closure. It re-runs this closure whenever any signals it depends on (reads)
  are updated
* The `Resource` object returned can be in several states when read:

1. `None` if the resource is still loading
2. `Some(value)` if the resource has successfully loaded

```rust
#[component]
fn Dog() -> Element {
    let mut dog = use_resource(move || async move { 42 });
    match dog() {
        Some(value) => rsx! { div { "Value: {value}" } },
        None => rsx! { div { "Loading..." } },
    }
}
```

# Routing

All possible routes are defined in a single Rust `enum` that derives `Routable`. Each variant represents a route and is
annotated with `#[route("/path")]`. Dynamic Segments can capture parts of the URL path as parameters by using `:name` in
the route string. These become fields in the enum variant.

The `Router<Route> {}` component is the entry point that manages rendering the correct component for the current URL.

You can use the `#[layout(NavBar)]` to create a layout shared between pages and place an `Outlet<Route> {}` inside your
layout component. The child routes will be rendered in the outlet.

```rust
#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[layout(NavBar)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    BlogPost { id: i32 },
}

#[component]
fn NavBar() -> Element {
    rsx! {
        a { href: "/", "Home" }
        Outlet::<Route> {}
    }
}

#[component]
fn App() -> Element {
    rsx! { Router::<Route> {} }
}
```

```toml
dioxus = { version = "0.7.0", features = ["router"] }
```

### Errors

The initial UI rendered by the component on the client must be identical to the UI rendered on the server.

* Use the `use_server_future` hook instead of `use_resource`. It runs the future on the server, serializes the result,
  and sends it to the client, ensuring the client has the data immediately for its first render.
* Any code that relies on browser-specific APIs (like accessing `localStorage`) must be run *after* hydration. Place
  this code inside a `use_effect` hook.

# SCSS standards for this project (cadence-ui)

Scope

- Styles for the UI live under `crates/cadence-ui/` with app-level assets in `assets/` and feature/component styles in
  `src/**`. The aggregator is `assets/ui.scss`.

Imports

- Prefer `@use` to import other SCSS modules. Keep module paths relative, matching the existing structure (see
  `assets/ui.scss`).
- Only use `@import` for the legacy `mixins.scss` where needed until it is migrated. Do not introduce new `@import` for
  general styles.

Variables and tokens

- Use color and spacing tokens from `assets/theme.scss`. Do not hardcode colors. Example: `$primary-color`,
  `$secondary-color-4`, `$navbar-height`, `$navbar-reserved-space`.
- Reuse mixins from `assets/mixins.scss` such as `row`, `column`, `text-primary` instead of hand-writing flex/text
  styles.

Naming and structure

- Use lowercase, hyphen-separated class names (e.g., `.navbar-container`, `.navbar-item-label`).
- IDs may be used sparingly for top-level layout containers that are already present in the codebase (e.g., `#main`,
  `#navbar`). Do not add new IDs for styling unless aligning with existing layout patterns.
- One component per file. Co-locate view-specific styles under `src/views/**` and shared components under
  `src/**/<component>/<component>.scss` as in the repository.

Nesting and specificity

- Limit nesting to a maximum of 3 levels. Avoid deep nesting chains that increase specificity.
- Prefer class-based selectors. Avoid tag selectors unless necessary for resets.

Layout and sizing

- Use logical viewport units like `dvh` and safe-area insets via `env(safe-area-inset-*)` consistent with existing
  files (`ui.scss`, `navbar.scss`).
- Avoid hard-coded heights unless part of a design token (e.g., `$navbar-height`).

Scrollbars

- Follow the pattern in `assets/ui.scss` using `@supports (scrollbar-width: auto)` and
  `@supports selector(::-webkit-scrollbar)` to provide cross-browser styling.

Animations

- Define keyframes at the bottom of the file or in a dedicated block, using descriptive names (see
  `@keyframes shrinkPulse`).

Do not add comments in SCSS

- Do not include `//` or `/* */` comments in SCSS you generate. Express intent through clear naming and structure.

# Project launch and tooling

- Use `dx serve` for development. When adding assets referenced by `asset!()`, place them under `/assets` at the project
  root and reference with absolute paths like `asset!("/assets/image.png")`.
