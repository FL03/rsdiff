/*
   Appellation: dim <mod>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Dimension
//!

pub use self::{axis::Axis, dimension::*, rank::Rank};

pub(crate) mod axis;
pub(crate) mod dimension;
pub(crate) mod rank;

pub trait IntoAxis {
    fn into_axis(self) -> Axis;
}

impl IntoAxis for usize {
    fn into_axis(self) -> Axis {
        Axis::new(self)
    }
}

pub trait IntoRank {
    fn into_rank(self) -> Rank;
}

impl IntoRank for usize {
    fn into_rank(self) -> Rank {
        Rank::new(self)
    }
}

pub trait Dimension {}
