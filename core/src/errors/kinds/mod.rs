/*
    Appellation: kinds <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{external::*, propagation::*, standard::*};

pub(crate) mod external;
pub(crate) mod propagation;
pub(crate) mod standard;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use strum::{Display, EnumCount, EnumIs, EnumIter, VariantNames};

pub trait ErrorType {
    type Kind: std::fmt::Display;

    fn kind(&self) -> &Self::Kind;

    fn name(&self) -> String;
}

#[derive(
    Clone,
    Copy,
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
pub enum ErrorKind<E = String> {
    #[default]
    External(ExternalError<E>),
    Sync(SyncError),
}
