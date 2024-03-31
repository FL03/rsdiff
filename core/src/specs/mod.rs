/*
    Appellation: specs <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub use self::{arith::*, gradient::*, prop::*, store::*};

pub(crate) mod arith;
pub(crate) mod gradient;
pub(crate) mod prop;
pub(crate) mod store;

pub mod func;

use core::borrow::Borrow;

pub trait Idx {
    type Index;
}

pub trait IdxExt: Idx
where
    Self: Borrow<Self::Index> + Copy,
{
}

pub(crate) mod prelude {
    pub use super::func::*;
    pub use super::gradient::*;
    pub use super::prop::*;
    pub use super::store::*;
}

#[cfg(test)]
mod tests {}
