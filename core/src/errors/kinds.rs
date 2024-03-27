/*
    Appellation: error <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use strum::{Display, EnumCount, EnumIs, EnumIter, VariantNames};

pub trait ErrorType {
    type Kind;

    fn kind(&self) -> &Self::Kind;

    fn name(&self) -> &str;
}

#[derive(
    Clone,
    Debug,
    Display,
    EnumCount,
    EnumIs,
    EnumIter,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    SmartDefault,
    VariantNames,
)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize,),
    serde(rename_all = "snake_case")
)]
#[strum(serialize_all = "snake_case")]
pub enum ErrorKind {
    #[default]
    External(ExternalError),
    Sync(SyncError),
}

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

impl<E, K> ErrorType for ExternalError<E>
where
    E: ErrorType<Kind = K>,
{
    type Kind = ExternalError<E>;

    fn kind(&self) -> &Self::Kind {
        &self
    }

    fn name(&self) -> &str {
        match self {
            Self::Custom(inner) => inner.name(),
            Self::Unknown => "unknown",
        }
    }
}

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
pub enum StdError {
    #[default]
    IO,
    Parse,
    Sync(SyncError),
}

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
pub enum SyncError {
    #[default]
    Poison,
    TryLock,
}
