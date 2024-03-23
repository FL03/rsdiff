/*
    Appellation: order <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIs, EnumIter, EnumString, VariantNames};

#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize,),
    serde(rename_all = "snake_case", untagged)
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
#[strum(serialize_all = "snake_case")]
pub enum Order {
    Column,
    #[default]
    Row,
}

impl Order {
    pub fn column() -> Self {
        Self::Column
    }

    pub fn row() -> Self {
        Self::Row
    }
}

impl From<Order> for usize {
    fn from(order: Order) -> Self {
        order as usize
    }
}

impl From<usize> for Order {
    fn from(order: usize) -> Self {
        match order % Self::COUNT {
            0 => Self::Column,
            _ => Self::Row,
        }
    }
}
