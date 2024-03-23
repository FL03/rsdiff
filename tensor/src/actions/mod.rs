/*
    Appellation: actions <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Actions
//!
//!

pub mod arange;
pub mod grad;
pub mod index;

pub(crate) mod prelude {
    pub use super::arange::*;
}

#[cfg(test)]
mod tests {}
