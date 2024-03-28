/*
   Appellation: axis <mod>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Axis
//!
//! 
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Axis(pub(crate) usize);

impl Axis {
    pub fn new(axis: usize) -> Self {
        Axis(axis)
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