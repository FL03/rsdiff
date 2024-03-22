/*
   Appellation: axis <mod>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Axis
//!

pub struct Axis(pub(crate) usize);

impl Axis {
    pub fn new(axis: usize) -> Self {
        Axis(axis)
    }

    pub fn axis(&self) -> usize {
        self.0
    }
}
