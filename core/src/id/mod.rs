/*
    Appellation: ids <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Ids
//!
//!
pub use self::{id::Id, kinds::*};

pub(crate) mod id;

pub(crate) mod kinds {
    pub use self::atomic::AtomicId;

    pub(crate) mod atomic;
}

pub trait Identifier {}

pub trait Identifiable {
    type Id: Identifier;

    fn id(&self) -> Self::Id;
}

#[cfg(test)]
mod tests {}
