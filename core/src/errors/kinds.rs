/*
    Appellation: error <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIs, EnumIter, VariantNames};

pub trait ErrorType {
    type Kind;

    fn kind(&self) -> Self::Kind;
}

pub enum Errors<T> {
    Specific(Box<dyn ErrorType<Kind = T>>),
    Unknown,
}

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Display,
    EnumCount,
    EnumIs,
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
pub enum ErrorKind {
    Func,
    Graph,
    Sync,
    #[default]
    Unknown,
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
pub enum ExternalError<E> {
    Known(E),
    #[default]
    Unknown,
}

impl<E> ExternalError<E> {
    pub fn new(error: E) -> Self {
        Self::Known(error)
    }

    pub fn unknown() -> Self {
        Self::Unknown
    }
}
