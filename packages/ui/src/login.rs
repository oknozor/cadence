//! Login component for Subsonic authentication

use dioxus::prelude::*;

#[component]
pub fn Login(on_login: EventHandler<(String, String, String)>) -> Element {
    let mut server_url = use_signal(|| String::new());
    let mut username = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());

    rsx! {
        div {
            class: "login-container",
            h1 { "Connect to Subsonic Server" }

            form {
                onsubmit: move |e| {
                    e.prevent_default();
                    on_login.call((
                        server_url.read().clone(),
                        username.read().clone(),
                        password.read().clone(),
                    ));
                },

                div {
                    class: "form-group",
                    label { "Server URL:" }
                    input {
                        r#type: "text",
                        placeholder: "https://demo.subsonic.org",
                        value: "{server_url}",
                        oninput: move |e| server_url.set(e.value().clone()),
                    }
                }

                div {
                    class: "form-group",
                    label { "Username:" }
                    input {
                        r#type: "text",
                        placeholder: "username",
                        value: "{username}",
                        oninput: move |e| username.set(e.value().clone()),
                    }
                }

                div {
                    class: "form-group",
                    label { "Password:" }
                    input {
                        r#type: "password",
                        placeholder: "password",
                        value: "{password}",
                        oninput: move |e| password.set(e.value().clone()),
                    }
                }

                button {
                    r#type: "submit",
                    "Connect"
                }
            }
        }
    }
}
