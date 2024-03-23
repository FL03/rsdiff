/*
    Appellation: grad <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Gradient
//!
//!
pub use self::iterator::Iterator;

pub(crate) mod iterator;

pub trait TensorIter {
    type Item;
}

#[cfg(test)]
mod tests {}
