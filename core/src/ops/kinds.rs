/*
    Appellation: kinds <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::arithmetic::*;
use super::BinaryOperation;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use strum::{Display, EnumCount, EnumIs, EnumIter, VariantNames};

#[derive(Clone)]
pub enum Op<T> {
    Binary(T, T, BinaryOp),
    Compare(T, T, CompareOp),
    Custom(String),
    Unary(T, UnaryOp),
}

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
    EnumIs,
    EnumIter,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    VariantNames,
)]
#[repr(u8)]
#[strum(serialize_all = "lowercase")]
pub enum CompareOp {
    #[default]
    Eq,
    Ge,
    Gt,
    Le,
    Lt,
    Ne,
}

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
    derive(Deserialize, Serialize,),
    serde(rename_all = "lowercase", untagged)
)]
#[repr(u8)]
#[strum(serialize_all = "lowercase")]
pub enum BinaryOp {
    #[default]
    Add(Addition),
    Div(Division),
    Maximum,
    Minimum,
    Mul(Multiplication),
    Sub(Subtraction),
}

impl BinaryOp {
    pub fn add() -> Self {
        Self::Add(Addition)
    }

    pub fn div() -> Self {
        Self::Div(Division)
    }

    pub fn maximum() -> Self {
        Self::Maximum
    }

    pub fn minimum() -> Self {
        Self::Minimum
    }

    pub fn mul() -> Self {
        Self::Mul(Multiplication)
    }

    pub fn sub() -> Self {
        Self::Sub(Subtraction)
    }

    pub fn is_commutative(&self) -> bool {
        match self {
            Self::Add(_) | Self::Mul(_) => true,
            _ => false,
        }
    }
}

impl<T> BinaryOperation<T, T> for BinaryOp
where
    T: Copy + Default + PartialOrd + num::traits::NumOps,
{
    type Output = T;

    fn eval(&self, lhs: T, rhs: T) -> Self::Output {
        match self {
            Self::Add(_) => lhs + rhs,
            Self::Div(_) => lhs / rhs,
            Self::Maximum => {
                if lhs > rhs {
                    lhs
                } else {
                    rhs
                }
            }
            Self::Minimum => {
                if lhs < rhs {
                    lhs
                } else {
                    rhs
                }
            }
            Self::Mul(_) => lhs * rhs,
            Self::Sub(_) => lhs - rhs,
        }
    }
}

impl From<Addition> for BinaryOp {
    fn from(_: Addition) -> Self {
        Self::Add(Addition)
    }
}

impl From<Division> for BinaryOp {
    fn from(_: Division) -> Self {
        Self::Div(Division)
    }
}

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Display,
    EnumCount,
    EnumIs,
    EnumIter,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    VariantNames,
)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize,),
    serde(rename_all = "lowercase", untagged)
)]
#[repr(u8)]
#[strum(serialize_all = "lowercase")]
pub enum UnaryOp {
    #[default]
    Abs,
    Ceil,
    Cos,
    Cosh,
    Exp,
    Inverse, // or Reciprocal
    Floor,
    Log,
    Neg,
    Round,
    Rsqrt,
    Sin,
    Sinh,
    Sqrt,
    Tan,
    Tanh,
}

#[derive(
    Clone,
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
    derive(Deserialize, Serialize,),
    serde(rename_all = "lowercase", untagged)
)]
#[repr(u8)]
#[strum(serialize_all = "lowercase")]
pub enum Ops {
    Binary(BinaryOp),
    Compare(CompareOp),
    #[default]
    Unary(UnaryOp),
    Custom {
        name: String,
    },
}

impl Ops {
    /// A functional constructor for [Ops::Binary]
    pub fn binary(op: BinaryOp) -> Self {
        Self::Binary(op)
    }
    /// A functional constructor for [Ops::Compare]
    pub fn compare(op: CompareOp) -> Self {
        Self::Compare(op)
    }
    /// A functional constructor for [Ops::Custom]
    pub fn custom(name: impl Into<String>) -> Self {
        Self::Custom { name: name.into() }
    }
    /// A functional constructor for [Ops::Unary]
    pub fn unary(op: UnaryOp) -> Self {
        Self::Unary(op)
    }
}

impl From<BinaryOp> for Ops {
    fn from(op: BinaryOp) -> Self {
        Self::Binary(op)
    }
}

impl From<CompareOp> for Ops {
    fn from(op: CompareOp) -> Self {
        Self::Compare(op)
    }
}

impl From<UnaryOp> for Ops {
    fn from(op: UnaryOp) -> Self {
        Self::Unary(op)
    }
}
