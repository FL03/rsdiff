/*
    Appellation: kinds <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{external::*, propagation::*, standard::*, types::*};

pub(crate) mod external;
pub(crate) mod propagation;
pub(crate) mod standard;
pub(crate) mod types;

use smart_default::SmartDefault;
use strum::{Display, EnumCount, EnumDiscriminants, EnumIs, EnumIter, VariantNames};

pub trait ErrorType {
    type Kind: core::fmt::Display;

    fn kind(&self) -> &Self::Kind;

    fn name(&self) -> String;
}

#[derive(
    Clone,
    Copy,
    Debug,
    Display,
    EnumCount,
    EnumDiscriminants,
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
    derive(serde::Deserialize, serde::Serialize,),
    serde(rename_all = "snake_case")
)]
#[strum(serialize_all = "snake_case")]
pub enum ErrorKind<E = String> {
    #[default]
    External(ExternalError<E>),
    Sync(SyncError),
}

#[cfg(feature = "std")]
impl<E> std::error::Error for ErrorKind<E> where E: core::fmt::Debug {}
