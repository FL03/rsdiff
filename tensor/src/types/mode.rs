/*
    Appellation: mode <module>
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
#[repr(u8)]
#[strum(serialize_all = "lowercase")]
pub enum TensorMode {
    #[default]
    Normal,
    Variable,
}

impl TensorMode {
    pub fn normal() -> Self {
        Self::Normal
    }

    pub fn variable() -> Self {
        Self::Variable
    }
}

impl From<TensorMode> for usize {
    fn from(mode: TensorMode) -> Self {
        mode as usize
    }
}

impl From<usize> for TensorMode {
    fn from(mode: usize) -> Self {
        match mode % Self::COUNT {
            0 => Self::Normal,
            _ => Self::Variable,
        }
    }
}

impl From<bool> for TensorMode {
    fn from(is_variable: bool) -> Self {
        if is_variable {
            Self::Variable
        } else {
            Self::Normal
        }
    }
}
