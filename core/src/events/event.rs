/*
   Appellation: event <module>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use crate::{EventSpec, Eventful};
use decanter::prelude::{Hash, Hashable};
use scsys::prelude::{Message, Timestamp};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Event<E: ToString = super::Events> {
    pub event: E,
    pub message: Message,
    pub timestamp: i64,
}

impl<E: ToString> Event<E> {
    pub fn new(event: E, message: Option<Message>) -> Self {
        let timestamp = Timestamp::default().into();
        Self {
            event,
            message: message.unwrap_or_default(),
            timestamp,
        }
    }
}

impl<E: ToString> std::fmt::Display for Event<E>
where
    E: Serialize,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            scsys::fnl_remove(serde_json::to_string(&self).unwrap())
        )
    }
}

impl<E: ToString> Default for Event<E>
where
    E: Default,
{
    fn default() -> Self {
        Self::new(Default::default(), None)
    }
}

impl<E: ToString> Eventful for Event<E>
where
    E: Clone,
{
    type Event = E;

    fn event(&self) -> Self::Event {
        self.event.clone()
    }
}

impl<E: ToString> EventSpec for Event<E>
where
    E: Clone,
{
    type Data = serde_json::Value;

    fn message(&self) -> &scsys::prelude::Message<Self::Data> {
        &self.message
    }
}

impl<E: ToString> From<E> for Event<E> {
    fn from(data: E) -> Self {
        Self::new(data, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::Events;

    #[test]
    fn test_events_default() {
        let a = Event::<Events>::default();
        a.hash();
        assert_eq!(a.event(), crate::events::Events::None);
    }
}
