/*
    Appellation: errors <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use ndarray::ShapeError;
use strum::{Display, EnumCount, EnumIs, VariantNames};
pub type TensorResult<T = ()> = core::result::Result<T, TensorError>;

#[derive(
    Clone, Debug, Display, EnumCount, EnumIs, Eq, Hash, Ord, PartialEq, PartialOrd, VariantNames,
)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "snake_case", untagged)
)]
#[repr(usize)]
#[strum(serialize_all = "snake_case")]
pub enum TensorError {
    Shape(String),
    Unknown(String),
}

impl std::error::Error for TensorError {}

impl From<&str> for TensorError {
    fn from(error: &str) -> Self {
        TensorError::Unknown(error.to_string())
    }
}

impl From<String> for TensorError {
    fn from(error: String) -> Self {
        TensorError::Unknown(error)
    }
}

macro_rules! into_tensor_error {
    ($($n:tt),*) => {
        into_tensor_error!(@loop $($n),*);
    };
    (@loop $(($kind:ident, $err:ident)),*) => {
        into_tensor_error!(@loop $($kind, $err)*);
    };
    (@loop $(($kind:ident, $err:ident, $call:ident)),*) => {
        into_tensor_error!(@loop $($kind, $err, $call)*);
    };

    (@loop $kind:ident, $error:ident, $call:ident) => {
        impl From<$error> for TensorError {
            fn from(error: $error) -> Self {
                TensorError::$kind(error.$call())
            }
        }
    };
    (@loop $kind:ident, $error:ident) => {
        impl From<$error> for TensorError {
            fn from(error: $error) -> Self {
                TensorError::$kind(error)
            }
        }
    };
}

into_tensor_error!((Shape, ShapeError, to_string));
