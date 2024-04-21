/*
    Appellation: external <module>
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
    Default,
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
pub enum ExternalError<E = String> {
    Custom(E),
    #[default]
    Unknown,
}

impl<E> ExternalError<E> {
    pub fn new(error: E) -> Self {
        Self::Custom(error)
    }

    pub fn unknown() -> Self {
        Self::Unknown
    }
}

impl<E> std::error::Error for ExternalError<E> where E: std::fmt::Debug {}

impl<E> ErrorType for ExternalError<E>
where
    E: ToString,
{
    type Kind = ExternalError<E>;

    fn kind(&self) -> &Self::Kind {
        self
    }

    fn name(&self) -> String {
        match self {
            Self::Custom(inner) => inner.to_string(),
            _ => self.to_string(),
        }
    }
}
