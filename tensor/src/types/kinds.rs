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
pub enum TensorKind {
    #[default]
    Normal,
    Variable,
}

impl TensorKind {
    pub fn normal() -> Self {
        Self::Normal
    }

    pub fn variable() -> Self {
        Self::Variable
    }
}

impl From<TensorKind> for usize {
    fn from(mode: TensorKind) -> Self {
        mode as usize
    }
}

impl From<usize> for TensorKind {
    fn from(mode: usize) -> Self {
        match mode % Self::COUNT {
            0 => Self::Normal,
            _ => Self::Variable,
        }
    }
}

impl From<bool> for TensorKind {
    fn from(is_variable: bool) -> Self {
        if is_variable {
            Self::Variable
        } else {
            Self::Normal
        }
    }
}
