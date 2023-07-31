/*
   Appellation: events <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # events
pub use self::event::*;

pub(crate) mod event;

pub trait Eventful {
   type Event: EventSpec;
}

pub trait EventSpec {

}