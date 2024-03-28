/*
   Appellation: dim <mod>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Dimension
//!

pub use self::dimension::Dim;

pub(crate) mod dimension;

pub trait Dimension {
    type Pattern;

    fn elements(&self) -> usize;

    fn ndim(&self) -> usize;
}
