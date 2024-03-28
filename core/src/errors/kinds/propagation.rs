/*
    Appellation: propagation <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::ErrorType;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIs, VariantNames};

#[derive(
    Clone,
    Copy,
    Debug,
    Display,
    EnumCount,
    EnumIs,
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
pub enum ModuleError {
    Predict(PredictError),
}

impl ErrorType for ModuleError {
    type Kind = ModuleError;

    fn kind(&self) -> &Self::Kind {
        self
    }

    fn name(&self) -> String {
        self.to_string()
    }
}

#[derive(
    Clone,
    Copy,
    Debug,
    Display,
    EnumCount,
    EnumIs,
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
pub enum PredictError {
    Arithmetic,
    NumericalError,
}

#[derive(
    Clone,
    Copy,
    Debug,
    Display,
    EnumCount,
    EnumIs,
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
pub enum GradientError {
    Backward,
    Forward,
}
