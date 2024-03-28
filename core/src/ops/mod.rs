/*
    Appellation: ops <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Operations
//!
//!
pub use self::kinds::*;

pub(crate) mod kinds;

pub mod binary;
pub mod unary;

pub trait Operation {
    type Output;

    fn kind(&self) -> String;
}

pub trait Pow<T> {
    type Output;

    fn pow(&self, exp: T) -> Self::Output;
}

pub trait Powc<T>: Pow<T> {
    fn powc(&self, exp: T) -> Self::Output;
}

pub trait Powi<T>: Pow<T> {
    fn powi(&self, exp: T) -> Self::Output;
}

pub trait Powf<T>: Pow<T>
where
    T: num::Float,
{
    fn powf(&self, exp: T) -> Self::Output;
}

pub trait Squared {
    type Output;

    fn squared(&self) -> Self::Output;
}

pub(crate) mod prelude {
    pub use super::binary::*;
    pub use super::kinds::Op;
    pub use super::unary::*;
    pub use super::Operation;
}
