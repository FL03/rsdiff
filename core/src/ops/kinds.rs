/*
    Appellation: kinds <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::binary::BinaryOp;
use super::unary::UnaryOp;
use strum::{Display, EnumCount, EnumIs, VariantNames};

#[derive(
    Clone,
    Copy,
    Debug,
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
    derive(serde::Deserialize, serde::Serialize,),
    serde(rename_all = "lowercase", untagged),
    strum(serialize_all = "lowercase")
)]
pub enum Op {
    Binary(BinaryOp),
    Unary(UnaryOp),
}

impl From<BinaryOp> for Op {
    fn from(op: BinaryOp) -> Self {
        Self::Binary(op)
    }
}

impl From<UnaryOp> for Op {
    fn from(op: UnaryOp) -> Self {
        Self::Unary(op)
    }
}
