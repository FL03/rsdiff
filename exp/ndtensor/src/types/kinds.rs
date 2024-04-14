/*
    Appellation: mode <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use strum::{Display, EnumCount, EnumIs, EnumIter, EnumString, VariantNames};

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
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize,),
    serde(rename_all = "lowercase", untagged)
)]
#[repr(u8)]
#[strum(serialize_all = "lowercase")]
pub enum TensorKind {
    #[default]
    Normal = 0,
    Variable = 1,
}

impl TensorKind {
    pub fn new(kind: bool) -> Self {
        match kind {
            true => Self::Variable,
            false => Self::Normal,
        }
    }
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
