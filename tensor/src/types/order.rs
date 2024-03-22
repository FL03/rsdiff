/*
    Appellation: order <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIter, EnumString, VariantNames};

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
pub enum MajorOrder {
    Column,
    #[default]
    Row,
}

impl MajorOrder {
    pub fn column() -> Self {
        Self::Column
    }

    pub fn row() -> Self {
        Self::Row
    }
}

impl From<MajorOrder> for usize {
    fn from(order: MajorOrder) -> Self {
        order as usize
    }
}

impl From<usize> for MajorOrder {
    fn from(order: usize) -> Self {
        match order % Self::COUNT {
            0 => Self::Column,
            _ => Self::Row,
        }
    }
}
