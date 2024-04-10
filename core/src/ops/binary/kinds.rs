/*
    Appellation: kinds <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::arithmetic::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use strum::{Display, EnumCount, EnumIs, EnumIter, VariantNames};

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
    EnumIter,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    SmartDefault,
    VariantNames,
)]
#[repr(u8)]
#[strum(serialize_all = "lowercase")]
pub enum BinaryOp {
    // <Kind = String> {
    #[default]
    Add(Addition),
    Div(Division),
    Mul(Multiplication),
    Sub(Subtraction),
    Pow,
    Rem(Remainder),
    Max,
    Min,
    And,
    Or,
    Xor,
    Shl,
    Shr,
    Custom(),
}

impl BinaryOp {
    pub fn differentiable(&self) -> bool {
        match self {
            BinaryOp::Add(_) | BinaryOp::Div(_) | Self::Mul(_) | Self::Sub(_) | BinaryOp::Pow => {
                true
            }
            _ => false,
        }
    }

    pub fn is_commutative(&self) -> bool {
        match self {
            BinaryOp::Add(_) | Self::Mul(_) | BinaryOp::And | BinaryOp::Or | BinaryOp::Xor => true,
            _ => false,
        }
    }

    simple_enum_constructor!(
        (Add, add, Addition),
        (Div, div, Division),
        (Mul, mul, Multiplication),
        (Rem, rem, Remainder),
        (Sub, sub, Subtraction)
    );
    variant_constructor!(
        (Pow, pow),
        (Max, max),
        (Min, min),
        (And, bitand),
        (Or, bitor),
        (Xor, bitxor),
        (Shl, shl),
        (Shr, shr)
    );
}
