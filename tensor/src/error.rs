/*
    Appellation: error <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::shape::error::ShapeError;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIs, EnumIter, EnumString, VariantNames};

pub type TensorResult<T = ()> = std::result::Result<T, TensorError>;

#[derive(
    Clone, Debug, Display, EnumCount, EnumIs, Eq, Hash, Ord, PartialEq, PartialOrd, VariantNames,
)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case", untagged)
)]
#[repr(usize)]
#[strum(serialize_all = "snake_case")]
pub enum TensorError {
    Arithmetic(ArithmeticError),
    Indexing(String),
    Shape(ShapeError),
    Singular,
}

unsafe impl Send for TensorError {}

unsafe impl Sync for TensorError {}

impl std::error::Error for TensorError {}

impl From<&str> for TensorError {
    fn from(error: &str) -> Self {
        TensorError::Indexing(error.to_string())
    }
}

#[derive(
    Clone,
    Copy,
    Debug,
    Display,
    EnumCount,
    EnumIs,
    EnumIter,
    EnumString,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    VariantNames,
)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case", untagged)
)]
#[repr(usize)]
#[strum(serialize_all = "snake_case")]
pub enum ArithmeticError {
    DivisionByZero,
    Overflow,
    Underflow,
}

macro_rules! into_tensor_error {
    ($error:ident, $kind:ident) => {
        impl From<$error> for TensorError {
            fn from(error: $error) -> Self {
                TensorError::$kind(error)
            }
        }
    };
}

into_tensor_error!(ArithmeticError, Arithmetic);
into_tensor_error!(ShapeError, Shape);
