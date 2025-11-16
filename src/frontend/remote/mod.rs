use dioxus::prelude::*;

static CSS: Asset = asset!("/assets/remote.css");
static CURSOR: Asset = asset!("/assets/cursor.png");

#[component]
pub fn Remote() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }
        div { id: "content",
            div { id: "screen",
                img { id: "screen-img", src: "/api/remote/screencast" }
                img { id: "screen-cursor", src: CURSOR }
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
