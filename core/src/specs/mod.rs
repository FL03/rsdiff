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

pub trait AsSlice<T> {
    fn as_slice(&self) -> &[T];
}

impl<S, T> AsSlice<T> for S
where
    S: AsRef<[T]>,
{
    fn as_slice(&self) -> &[T] {
        self.as_ref()
    }
}

pub trait AsSliceMut<T> {
    fn as_slice_mut(&mut self) -> &mut [T];
}

impl<S, T> AsSliceMut<T> for S
where
    S: AsMut<[T]>,
{
    fn as_slice_mut(&mut self) -> &mut [T] {
        self.as_mut()
    }
}

pub(crate) mod prelude {
    pub use super::func::*;
    pub use super::gradient::*;
    pub use super::prop::*;
    pub use super::store::*;
}

#[cfg(test)]
mod tests {}
