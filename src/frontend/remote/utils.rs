use std::ops::{Deref, DerefMut};

use dioxus_fullstack::{PostcardEncoding, UseWebsocket};

use super::{Interaction, RelativePosition};

type UseWebsocketInstance = UseWebsocket<Interaction, RelativePosition, PostcardEncoding>;

#[derive(Clone, Copy)]
/// A [`UseWebsocket`] that implements [`PartialEq`].
pub struct EqWebsocket(UseWebsocketInstance);

impl EqWebsocket {
    pub fn new(socket: UseWebsocketInstance) -> Self {
        Self(socket)
    }
}

// no two websockets are ever equal
impl PartialEq for EqWebsocket {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

impl Deref for EqWebsocket {
    type Target = UseWebsocketInstance;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for EqWebsocket {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
