/*
    Appellation: types <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::error::ErrorType;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIs, EnumIter, EnumString, VariantNames};

#[derive(
    Clone,
    Copy,
    Debug,
    Display,
    EnumCount,
    EnumIs,
    EnumString,
    EnumIter,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    VariantNames,
)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize,),
    serde(rename_all = "snake_case")
)]
#[strum(serialize_all = "snake_case")]
pub enum TypeError {
    ConversionError,
    InferenceError,
    InvalidType,
}

impl std::error::Error for TypeError {}

impl ErrorType for TypeError {
    type Kind = TypeError;

    fn kind(&self) -> &Self::Kind {
        self
    }

    fn name(&self) -> String {
        self.to_string()
    }
}
