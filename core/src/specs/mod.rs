/*
    Appellation: specs <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub use self::{gradient::*, store::*};

pub(crate) mod gradient;
pub(crate) mod store;

pub mod func;

use crate::errors::PredictError;

pub trait Backward {
    type Output;

    fn backward(&self) -> Self::Output;
}

pub trait Forward<T> {
    type Output;

    fn forward(&self, args: &T) -> Result<Self::Output, PredictError>;
}

impl<S, T> Forward<T> for Option<S>
where
    S: Forward<T, Output = T>,
    T: Clone,
{
    type Output = T;

    fn forward(&self, args: &T) -> Result<Self::Output, PredictError> {
        match self {
            Some(s) => s.forward(args),
            None => Ok(args.clone()),
        }
    }
}

pub(crate) mod prelude {
    pub use super::func::*;
    pub use super::gradient::*;
    pub use super::store::*;
}

#[cfg(test)]
mod tests {}
