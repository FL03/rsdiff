/*
   Appellation: axis <mod>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
use core::ops::Deref;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub trait IntoAxis {
    fn into_axis(self) -> Axis;
}

impl IntoAxis for usize {
    fn into_axis(self) -> Axis {
        Axis::new(self)
    }
}

/// An [Axis] is used to represent a dimension in a tensor.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Axis(pub(crate) usize);

impl Axis {
    pub fn new(axis: usize) -> Self {
        Self(axis)
    }

    pub fn into_inner(self) -> usize {
        self.0
    }

    pub fn axis(&self) -> usize {
        self.0
    }
}

impl AsRef<usize> for Axis {
    fn as_ref(&self) -> &usize {
        &self.0
    }
}

impl Deref for Axis {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for Axis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<usize> for Axis {
    fn from(axis: usize) -> Self {
        Axis(axis)
    }
}

impl From<Axis> for usize {
    fn from(axis: Axis) -> Self {
        axis.0
    }
}
