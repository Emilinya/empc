use dioxus::prelude::*;

#[component]
pub fn Shutdown() -> Element {
    rsx! {
        meta { http_equiv: "refresh", content: "2;url=http://empc.emilie.moe/" }
        div { id: "parts",
            form { class: "part",
                div { class: "name", "URL or Magnet Link:" }
                input { disabled: true, r#type: "url" }
                button { disabled: true, "Play" }
            }
            form { class: "part",
                div { class: "name", "File:" }
                input { disabled: true, r#type: "file" }
                button { disabled: true, "Play" }
            }
            form { class: "part",
                button { disabled: true, class: "link", "Remote" }
            }
            form { class: "part",
                button { disabled: true, class: "link", "Local Media" }
            }
            form { class: "part",
                button { disabled: true, class: "link", "Shutting down..." }
            }
        }
    }
}
