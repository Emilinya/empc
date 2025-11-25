use dioxus::prelude::*;

use crate::frontend::remote::Key;

use super::{EqWebsocket, Interaction};

#[component]
pub fn Controls(socket: EqWebsocket) -> Element {
    let mut input = use_signal(String::new);

    rsx! {
        div { id: "controls",
            form {
                id: "keyboard-form",
                onsubmit: move |event| async move {
                    event.prevent_default();

                    if let Err(err) = socket.send(Interaction::Text(input())).await {
                        warn!("Failed to send {:?} to socket: {}", event, err);
                    }
                    *input.write() = String::new();
                },
                input {
                    autocapitalize: "none",
                    autocomplete: "off",
                    id: "keyboard",
                    oninput: move |event| {
                        event.prevent_default();
                        if event.data.valid() {
                            *input.write() = event.data.value();
                        }
                    },
                    value: input,
                }
            }
            button {
                id: "space",
                onclick: move |_| async move {
                    if let Err(err) = socket.send(Interaction::Key(Key::Space)).await {
                        warn!("Failed to send Key::Space to socket: {}", err);
                    }
                },
                "Space"
            }
            button {
                id: "backspace",
                onclick: move |_| async move {
                    if let Err(err) = socket.send(Interaction::Key(Key::Backspace)).await {
                        warn!("Failed to send Key::Backspace to socket: {}", err);
                    }
                },
                "Backspace"
            }
            button {
                id: "escape",
                onclick: move |_| async move {
                    if let Err(err) = socket.send(Interaction::Key(Key::Escape)).await {
                        warn!("Failed to send Key::Escape to socket: {}", err);
                    }
                },
                "Escape"
            }
        }
    }
}
