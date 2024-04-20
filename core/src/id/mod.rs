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
    ($($t:ty),*) => {
        $(
            impl_identifier!(@loop $t);
        )*
    };
    (@loop $t:ty) => {
        impl Identifier for $t {}
    };
}

impl_identifier! {
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
}

pub trait Identifiable {
    type Id: Identifier;

    fn id(&self) -> Self::Id;
}

#[cfg(test)]
mod tests {}
