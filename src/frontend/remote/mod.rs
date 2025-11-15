use dioxus::prelude::*;

#[component]
pub fn Remote() -> Element {
    rsx! {
        div { id: "content",
            div { id: "screen",
                img { id: "screen-img", src: "screenshot" }
                img { id: "screen-cursor", src: "cursor.png" }
            }
            div { id: "controls",
                form { id: "keyboard-form",
                    input {
                        autocapitalize: "none",
                        autocomplete: "off",
                        id: "keyboard",
                    }
                }
                button { id: "space", "Space" }
                button { id: "backspace", "Backspace" }
            }
        }
    }
}
