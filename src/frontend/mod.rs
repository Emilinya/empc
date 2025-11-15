mod local;
mod playback;
mod remote;

use {local::Local, playback::Playback, remote::Remote};

use dioxus::prelude::*;

const STYLE_CSS: Asset = asset!("/assets/style.css");
const SAKURA_CSS: Asset = asset!("/assets/sakura.css");

pub fn run() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: SAKURA_CSS }
        document::Link { rel: "stylesheet", href: STYLE_CSS }
        Router::<Route> {}
    }
}

#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/play")]
    Playback {},
    #[route("/remote")]
    Remote {},
    #[route("/local")]
    Local {},
}

#[component]
pub fn Home() -> Element {
    rsx! {
        div { id: "parts",
            form { class: "part", method: "post", action: "/play/url",
                div { class: "name", "URL or Magnet Link:" }
                input { autocomplete: "off", name: "url", r#type: "url" }
                button { "Play" }
            }
            form {
                class: "part",
                method: "post",
                action: "/play/file",
                enctype: "multipart/form-data",
                div { class: "name", "File:" }
                input { autocomplete: "off", name: "file", r#type: "file" }
                button { "Play" }
            }
            form { class: "part", method: "get", action: "/remote/",
                button { class: "link", "Remote" }
            }
            form { class: "part", method: "get", action: "/local/",
                button { class: "link", "Local Media" }
            }
        }
    }
}
