/*
    Appellation: standard <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIs, EnumIter, VariantNames};

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
pub enum SyncError {
    #[default]
    Poison,
    TryLock,
}
