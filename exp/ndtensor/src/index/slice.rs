/*
    Appellation: slice <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Slice
//!
//!
use core::ops::{Range, RangeFrom};
pub struct Slice {
    pub start: isize,
    pub end: Option<isize>,
    pub step: isize,
}

impl Slice {
    pub fn new(start: isize, end: Option<isize>, step: isize) -> Self {
        Self { start, end, step }
    }
}

impl From<Range<isize>> for Slice {
    fn from(range: Range<isize>) -> Self {
        Self {
            start: range.start,
            end: Some(range.end),
            step: 1,
        }
    }
}

impl From<RangeFrom<isize>> for Slice {
    fn from(range: RangeFrom<isize>) -> Self {
        Self {
            start: range.start,
            end: None,
            step: 1,
        }
    }
}

pub enum Slices {
    Index(isize),
    Slice(Slice),
    NewAxis,
}
