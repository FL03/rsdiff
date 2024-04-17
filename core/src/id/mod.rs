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

macro_rules! impl_identifier {
    (@loop $($t:ty),*) => {
        $(
            impl_identifier!(@loop $t);
        )*
    };
    (@loop $t:ty) => {
        impl Identifier for $t {}
    };
}

pub trait Identifiable {
    type Id: Identifier;

    fn id(&self) -> Self::Id;
}

#[cfg(test)]
mod tests {}
