/*
    Appellation: kinds <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::ops::ternary::ApplyTernary;
use crate::ops::{OpKind, Operator};
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
    name(TernaryOp),
    strum(serialize_all = "lowercase")
)]
pub enum TernaryExpr {
    Affine(Affine),
}

impl Operator for TernaryExpr {
    fn kind(&self) -> OpKind {
        OpKind::Ternary
    }

    fn name(&self) -> &str {
        match self {
            TernaryExpr::Affine(op) => op.name(),
        }
    }
}

impl Operator for TernaryOp {
    fn kind(&self) -> OpKind {
        OpKind::Ternary
    }

    fn name(&self) -> &str {
        match self {
            Self::Affine => "affine",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "lowercase")
)]
#[repr(C)]
pub struct Affine;

impl Operator for Affine {
    fn kind(&self) -> OpKind {
        OpKind::Ternary
    }

    fn name(&self) -> &str {
        "affine"
    }
}

impl<A, B, C> ApplyTernary<A, B, C> for Affine
where
    A: core::ops::Mul<B, Output = C>,
    C: core::ops::Add<Output = C>,
{
    type Output = C;

    fn apply(&self, a: A, b: B, c: C) -> Self::Output {
        a * b + c
    }
}
