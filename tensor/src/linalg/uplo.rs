/*
    Appellation: kinds <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIs, EnumIter, EnumString, VariantNames};

#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize,),
    serde(rename_all = "lowercase", untagged)
)]
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
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
#[repr(usize)]
#[strum(serialize_all = "lowercase")]
pub enum UPLO {
    Lower,
    #[default]
    Upper,
}

impl UPLO {
    pub fn lower() -> Self {
        Self::Lower
    }

    pub fn upper() -> Self {
        Self::Upper
    }
}

unsafe impl Send for UPLO {}

unsafe impl Sync for UPLO {}
