mod controls;
mod screen;
mod utils;

use dioxus::{
    fullstack::{WebSocketOptions, use_websocket},
    html::{
        geometry::{ElementPoint, Pixels, WheelDelta, euclid::Size2D},
        input_data,
    },
    prelude::*,
};
use serde::{Deserialize, Serialize};

use crate::backend::remote::interaction;

use controls::Controls;
use screen::Screen;
use utils::EqWebsocket;

static CSS: Asset = asset!("/assets/remote.css");

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Interaction {
    Position(RelativePosition),
    MouseDown(MouseButton),
    MouseUp(MouseButton),
    Scroll(f32),
    Text(String),
    Key(Key),
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

impl TryFrom<input_data::MouseButton> for MouseButton {
    type Error = anyhow::Error;

    fn try_from(value: input_data::MouseButton) -> anyhow::Result<Self> {
        match value {
            input_data::MouseButton::Primary => Ok(MouseButton::Left),
            input_data::MouseButton::Secondary => Ok(MouseButton::Right),
            input_data::MouseButton::Auxiliary => Ok(MouseButton::Middle),
            _ => anyhow::bail!("Got invalid mouse button {:?}", value),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct RelativePosition {
    pub x: f32,
    pub y: f32,
}

impl RelativePosition {
    pub fn from_absolute_position(coordinates: ElementPoint, size: Size2D<f64, Pixels>) -> Self {
        Self {
            x: (coordinates.x / size.width) as f32,
            y: (coordinates.y / size.height) as f32,
        }
    }

    pub fn into_absolute_position(self, size: Size2D<f64, Pixels>) -> ElementPoint {
        ElementPoint::new(
            f64::from(self.x) * size.width,
            f64::from(self.y) * size.height,
        )
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Key {
    Backspace,
    Escape,
    Space,
}

#[component]
pub fn Remote() -> Element {
    let socket = use_websocket(|| interaction(WebSocketOptions::new()));

    rsx! {
        document::Stylesheet { href: CSS }
        div { id: "content",
            Screen { socket: EqWebsocket::new(socket) }
            Controls { socket: EqWebsocket::new(socket) }
        }
    }
}
