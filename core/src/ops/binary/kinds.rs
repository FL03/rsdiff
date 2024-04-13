/*
    Appellation: kinds <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::arithmetic::*;
use smart_default::SmartDefault;
use strum::{Display, EnumCount, EnumIs, EnumIter, VariantNames};

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
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "lowercase", untagged),
    strum(serialize_all = "lowercase")
)]
#[repr(u8)]
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
    Custom {
        id: usize,
    },
}

pub struct CustomBinOp {
    pub id: usize,
}

impl CustomBinOp {
    pub fn new(id: usize) -> Self {
        Self { id }
    }
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
        (Add, add, Addition::new),
        (Div, div, Division::new),
        (Mul, mul, Multiplication::new),
        (Rem, rem, Remainder::new),
        (Sub, sub, Subtraction::new)
    );

    simple_enum_constructor!(
        st Custom, custom, { id: usize }
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
