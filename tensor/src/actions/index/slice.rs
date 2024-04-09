/*
    Appellation: slice <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Slice
//!
//!
use super::Ixs;
use core::ops::{Range, RangeFrom, RangeTo};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Slice {
    pub start: Ixs,
    pub end: Option<Ixs>,
    pub step: Ixs,
}

impl Slice {
    pub fn new(start: Ixs, end: Option<Ixs>, step: Ixs) -> Self {
        Self { start, end, step }
    }
}

impl From<Range<Ixs>> for Slice {
    fn from(range: Range<Ixs>) -> Self {
        Self {
            start: range.start,
            end: Some(range.end),
            step: 1,
        }
    }
}

impl From<RangeFrom<Ixs>> for Slice {
    fn from(range: RangeFrom<Ixs>) -> Self {
        Self {
            start: range.start,
            end: None,
            step: 1,
        }
    }
}

impl From<RangeTo<Ixs>> for Slice {
    fn from(range: RangeTo<Ixs>) -> Self {
        Self {
            start: 0,
            end: Some(range.end),
            step: 1,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize), serde(rename_all = "snake_case"))]
pub enum Slices {
    Index(Ixs),
    Slice(Slice),
    NewAxis,
}
