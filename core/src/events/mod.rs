/*
   Appellation: events <module>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
pub use self::{event::*, opts::*};

pub(crate) mod event;
pub(crate) mod opts;

pub(crate) mod specs {
    use scsys::prelude::{Message, Timestamp};

    /// Describes the most basic supported implementation of an event
    pub trait Eventful {
        type Event;

        fn event(&self) -> Self::Event;
    }

    /// Describes a more robust event schematic for use throughout the ecosystem
    pub trait EventSpec: Eventful {
        type Data: Clone;

        fn message(&self) -> &Message<Self::Data>;
        fn timestamp(&self) -> Timestamp {
            Timestamp::from(self.message().timestamp)
        }

        fn push(&mut self, data: Self::Data) -> &Self {
            self.message().clone().push(data);

            self
        }
    }

    pub trait EventSpecExt: EventSpec {
        fn signal(&self) -> String;
    }
}
