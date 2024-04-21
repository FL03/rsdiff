/*
    Appellation: ids <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Ids
//!
//!
pub use self::{entry::*, id::*};

pub(crate) mod entry;
pub(crate) mod id;

use acme::prelude::Identifier;

pub trait GraphIdx {
    type Idx: Identifier;
    fn new(index: Self::Idx) -> Self;
    fn id(&self) -> EntryId;
    fn index(&self) -> &Self::Idx;
}

#[cfg(test)]
mod tests {}
