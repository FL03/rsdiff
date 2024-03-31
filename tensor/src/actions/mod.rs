/*
    Appellation: actions <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Actions
//!
//! This module describes the actions that may be taken on or by a tensor.
//!
//! The actions include:<br>
//! * Automatic Differentiation
//! * Creation Routines
//! * Indexing
//! * Iteration

pub mod arange;
pub mod grad;
pub mod index;
pub mod iter;

use num::traits::{FromPrimitive, Num};

pub trait Linspace<T> {
    fn linspace(start: T, stop: T, steps: usize) -> Self;
}

impl<T> Linspace<T> for Vec<T>
where
    T: Copy + Default + FromPrimitive + Num + PartialOrd,
{
    fn linspace(start: T, stop: T, steps: usize) -> Self {
        let step = arange::step_size(start, stop, steps);
        let mut vec = Vec::with_capacity(steps);
        let mut value = start;
        for _ in 0..steps {
            vec.push(value);
            value = value + step;
        }
        vec
    }
}

pub(crate) mod prelude {
    pub use super::arange::*;
    pub use super::grad::*;
    pub use super::index::*;
    pub use super::iter::*;
}

#[cfg(test)]
mod tests {}
