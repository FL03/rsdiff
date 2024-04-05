/*
    Appellation: error <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIs, EnumIter, EnumProperty, EnumString, VariantNames};

pub type ShapeResult<T = ()> = std::result::Result<T, ShapeError>;

#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case", untagged)
)]
#[derive(
    Clone,
    Copy,
    Debug,
    Display,
    EnumCount,
    EnumIs,
    EnumIter,
    EnumProperty,
    EnumString,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    VariantNames,
)]
#[repr(usize)]
#[strum(serialize_all = "snake_case")]
pub enum ShapeError {
    #[strum(props(desc = "Dimension mismatch"))]
    DimensionMismatch,
    #[strum(props(desc = "incompatible shapes"))]
    IncompatibleShapes,
    IncompatibleLayout,
    InvalidAxis,
    InvalidShape,
    MismatchedElements,
    NotSquare,
    OutOfBounds,
    Overflow,
    Unsupported,
}

unsafe impl Send for ShapeError {}

unsafe impl Sync for ShapeError {}

impl std::error::Error for ShapeError {}
