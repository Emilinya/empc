use dioxus::prelude::*;

#[component]
pub fn Playback() -> Element {
    rsx! {
        div { id: "group-info",
            div { id: "is-playing", "Not Playing" }
            div { id: "progress-text" }
        }
        div { id: "group-bar",
            progress { id: "progress" }
        }
        div { id: "group-buttons",
            button { class: "small", id: "mute", "Mute" }
            button { class: "small", id: "skip-back", "<" }
            button { class: "small", id: "pause", "||" }
            button { class: "small", id: "skip-forward", ">" }
            button { class: "small", id: "exit", "Exit" }
        }
        div { id: "group-volume",
            div { class: "title",
                "Volume "
                span { id: "volume-text", "0" }
                "%"
            }
            input { id: "volume", r#type: "range" }
        }
        div { id: "group-sub-delay",
            div { class: "title",
                "\r\n                Subtitle Delay "
                span { id: "sub-delay", "0" }
                "s\r\n            "
            }
            button { class: "small", id: "sub-delay-less2", "-1" }
            button { class: "small", id: "sub-delay-less", "-0.1" }
            button { class: "small", id: "sub-delay-reset", "0" }
            button { class: "small", id: "sub-delay-more", "+0.1" }
            button { class: "small", id: "sub-delay-more2", "+1" }
        }
        div { id: "group-subtitles",
            div { class: "title",
                "\r\n                Subtitles\r\n                "
                button { "onclick": "document.getElementById('subtitles-upload').click()",
                    "\r\n                    Upload\r\n                "
                }
            }
            div { id: "subtitles-options" }
            form {
                action: "upload",
                class: "hidden",
                enctype: "multipart/form-data",
                method: "post",
                input {
                    id: "subtitles-upload",
                    name: "file",
                    "onchange": "this.parentNode.submit()",
                    r#type: "file",
                }
            }
        }
    }
}
