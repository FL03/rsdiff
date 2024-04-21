/*
    Appellation: kinds <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::arithmetic::*;
use crate::ops::{OpKind, Operator};
use smart_default::SmartDefault;
use strum::{Display, EnumCount, EnumIs, EnumIter, EnumString, VariantNames};

#[derive(
    Clone,
    Copy,
    Debug,
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
    SmartDefault,
    VariantNames,
)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "lowercase", untagged)
)]
#[non_exhaustive]
#[repr(C)]
#[strum(serialize_all = "lowercase")]
pub enum BinaryOp {
    // <Kind = String> {
    #[default]
    Arith(Arithmetic),
    Max,
    Min,
    And,
    Or,
    Xor,
    Shl,
    Shr,
    // Custom()
}

pub struct CustomOp {
    pub id: usize,
}

impl CustomOp {
    pub fn new(id: usize) -> Self {
        Self { id }
    }
}

impl BinaryOp {
    pub fn differentiable(&self) -> bool {
        match self {
            Self::Arith(_) => true,
            _ => false,
        }
    }

    pub fn is_commutative(&self) -> bool {
        match self {
            Self::Arith(arith) => arith.is_commutative(),
            BinaryOp::And | BinaryOp::Or | BinaryOp::Xor => true,
            _ => false,
        }
    }
    nested_constructor!(
        Arith<Arithmetic>,
        arithmetic,
        [add, div, mul, pow, rem, sub]
    );

    // simple_enum_constructor!(
    //     st Custom, custom, { id: usize }
    // );
    variant_constructor!(
        (Max, max),
        (Min, min),
        (And, bitand),
        (Or, bitor),
        (Xor, bitxor),
        (Shl, shl),
        (Shr, shr)
    );
}

impl Operator for BinaryOp {
    fn name(&self) -> &str {
        match self {
            Self::Arith(inner) => inner.name(),
            Self::Max => "max",
            Self::Min => "min",
            Self::And => "and",
            Self::Or => "or",
            Self::Xor => "xor",
            Self::Shl => "shl",
            Self::Shr => "shr",
        }
    }

    fn kind(&self) -> OpKind {
        OpKind::Binary
    }
}
