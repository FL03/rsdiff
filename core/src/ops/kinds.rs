/*
    Appellation: kinds <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{BinaryOp, Operator, TernaryOp, UnaryOp};
use strum::{Display, EnumCount, EnumDiscriminants, EnumIs, EnumIter, EnumString, VariantNames};

#[derive(
    Clone,
    Copy,
    Debug,
    Display,
    EnumCount,
    EnumDiscriminants,
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
    strum_discriminants(derive(serde::Deserialize, serde::Serialize))
)]
#[strum(serialize_all = "lowercase")]
#[strum_discriminants(
    derive(
        Display,
        EnumCount,
        EnumIs,
        EnumIter,
        EnumString,
        Hash,
        Ord,
        PartialOrd,
        VariantNames
    ),
    name(OpKind)
)]
pub enum Op {
    Binary(BinaryOp),
    Ternary(TernaryOp),
    Unary(UnaryOp),
}

impl Op {
    pub fn binary(op: BinaryOp) -> Self {
        Self::Binary(op)
    }

    pub fn ternary(op: TernaryOp) -> Self {
        Self::Ternary(op)
    }

    pub fn unary(op: UnaryOp) -> Self {
        Self::Unary(op)
    }
}

impl Operator for Op {
    fn name(&self) -> &str {
        match self {
            Self::Binary(op) => op.name(),
            Self::Ternary(op) => op.name(),
            Self::Unary(op) => op.name(),
        }
    }

    fn kind(&self) -> OpKind {
        match self {
            Self::Binary(op) => op.kind(),
            Self::Ternary(op) => op.kind(),
            Self::Unary(op) => op.kind(),
        }
    }
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
