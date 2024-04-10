/*
    Appellation: slice <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Slice
//!
//!
use super::Ixs;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Slice {
    pub start: Ixs,
    pub end: Option<Ixs>,
    pub step: Ixs,
}

impl Slice {
    pub fn new(start: Ixs, end: Option<Ixs>, step: Ixs) -> Self {
        debug_assert_ne!(step, 0, "step must be non-zero");
        Self { start, end, step }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "snake_case")
)]
pub enum Slices {
    Index(Ixs),
    Slice(Slice),
    NewAxis,
}

impl Slices {
    /// Create a new axis
    pub fn new_axis() -> Self {
        Self::NewAxis
    }
    /// Create a new index
    pub fn index(index: Ixs) -> Self {
        Self::Index(index)
    }
    /// Create a new slice
    pub fn slice(start: Ixs, end: Option<Ixs>, step: Ixs) -> Self {
        Self::Slice(Slice::new(start, end, step))
    }
}

pub struct Slicer {
    slices: Vec<Slices>,
}

impl Slicer {
    pub fn new(slices: impl IntoIterator<Item = Slices>) -> Self {
        Self {
            slices: Vec::from_iter(slices),
        }
    }

    pub fn iter(&self) -> core::slice::Iter<'_, Slices> {
        self.slices.iter()
    }
}

macro_rules! impl_from_range {
    ($self:ty, $constructor:expr, $idx:ty, [$($index:ty),*]) => {
        $(
            impl_from_range!($self, $constructor, $idx, $index);
        )*
    };
    ($self:ty, $constructor:expr, $idx:ty, $index:ty) => {
        impl From<core::ops::Range<$index>> for $self {
            #[inline]
            fn from(r: core::ops::Range<$index>) -> $self {
                $constructor(r.start as $idx, Some(r.end as $idx), 1)
            }
        }

        impl From<core::ops::RangeFrom<$index>> for $self {
            #[inline]
            fn from(r: core::ops::RangeFrom<$index>) -> $self {
                $constructor(r.start as $idx, None, 1)
            }
        }

        impl From<core::ops::RangeInclusive<$index>> for $self {
            #[inline]
            fn from(r: core::ops::RangeInclusive<$index>) -> $self {
                let end = *r.end() as $idx;
                $constructor(*r.start() as $idx, if end == -1 { None } else { Some(end + 1) }, 1)
            }
        }

        impl From<core::ops::RangeTo<$index>> for $self {
            #[inline]
            fn from(r: core::ops::RangeTo<$index>) -> $self {
                $constructor(0, Some(r.end as $idx), 1)
            }
        }

        impl From<core::ops::RangeToInclusive<$index>> for $self {
            #[inline]
            fn from(r: core::ops::RangeToInclusive<$index>) -> $self {
                let end = r.end as $idx;
                $constructor(0, if end == -1 { None } else { Some(end + 1) }, 1)
            }
        }
    };
}

impl_from_range!(Slice, Slice::new, Ixs, [i32, isize, usize]);
impl_from_range!(Slices, Slices::slice, Ixs, [i32, isize, usize]);
