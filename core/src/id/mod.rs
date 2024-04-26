/*
    Appellation: ids <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{id::IndexId, kinds::*};

pub(crate) mod id;

pub(crate) mod kinds {
    pub use self::atomic::AtomicId;

    pub mod atomic;
}

pub trait Identifier: ToString {}

pub trait Id<T> {
    type Id: core::borrow::Borrow<T> + Identifier;
}

pub trait IntoId {
    type Id: Identifier;

    fn into_id(self) -> Self::Id;
}

pub trait Identifiable: Identify {
    fn id(&self) -> &Self::Id;
}

pub trait Identify {
    type Id: Identifier;

    fn id(&self) -> &Self::Id;
}

pub trait IdentifyMut: Identify {
    fn id_mut(&mut self) -> &mut Self::Id;
}

impl<S> Identify for S
where
    S: Identifier,
{
    type Id = S;

    fn id(&self) -> &Self::Id {
        self
    }
}

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
    bool, char, f32, f64, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, &str, String
}

#[cfg(test)]
mod tests {}
