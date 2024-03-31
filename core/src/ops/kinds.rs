/*
    Appellation: kinds <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::binary::{BinaryOp, BinaryOperator};
use super::unary::UnaryOp;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIs, VariantNames};

#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize,),
    serde(rename_all = "lowercase", untagged)
)]
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
#[strum(serialize_all = "lowercase")]
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

pub enum Expr {
    Binary(BinaryOperator<Box<dyn std::any::Any>>),
}
