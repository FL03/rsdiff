/*
   Appellation: opts <events>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use crate::{events::Event, Eventful};
use scsys::prelude::Message;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Events {
    Custom(String),
    Startup,
    Shutdown,
    None,
}

impl Eventful for Events {
    type Event = Self;

    fn event(&self) -> Self::Event {
        self.clone()
    }
}

impl Default for Events {
    fn default() -> Self {
        Self::None
    }
}

impl std::fmt::Display for Events {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            scsys::fnl_remove(serde_json::to_string(&self).unwrap())
        )
    }
}

impl From<Events> for Message<Event<Events>> {
    fn from(data: Events) -> Self {
        Message::from(Event::from(data))
    }
}
