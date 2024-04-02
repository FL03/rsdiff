/*
    Appellation: kinds <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use acme::ops::binary::*;
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
pub enum CompareExpr {
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
pub enum BinaryExpr {
    #[default]
    Add(Addition),
    Div(Division),
    Maximum,
    Minimum,
    Mul(Multiplication),
    Sub(Subtraction),
}

impl BinaryExpr {
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

impl<T> BinaryOperation<T, T> for BinaryExpr
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

impl From<Addition> for BinaryExpr {
    fn from(_: Addition) -> Self {
        Self::Add(Addition)
    }
}

impl From<Division> for BinaryExpr {
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
pub enum UnaryExpr {
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
pub enum Operations {
    Binary(BinaryExpr),
    Compare(CompareExpr),
    #[default]
    Unary(UnaryExpr),
    Custom {
        name: String,
    },
}

impl Operations {
    /// A functional constructor for [Ops::Binary]
    pub fn binary(op: BinaryExpr) -> Self {
        Self::Binary(op)
    }
    /// A functional constructor for [Ops::Compare]
    pub fn compare(op: CompareExpr) -> Self {
        Self::Compare(op)
    }
    /// A functional constructor for [Ops::Custom]
    pub fn custom(name: impl Into<String>) -> Self {
        Self::Custom { name: name.into() }
    }
    /// A functional constructor for [Ops::Unary]
    pub fn unary(op: UnaryExpr) -> Self {
        Self::Unary(op)
    }
}

impl From<BinaryExpr> for Operations {
    fn from(op: BinaryExpr) -> Self {
        Self::Binary(op)
    }
}

impl From<CompareExpr> for Operations {
    fn from(op: CompareExpr) -> Self {
        Self::Compare(op)
    }
}

impl From<UnaryExpr> for Operations {
    fn from(op: UnaryExpr) -> Self {
        Self::Unary(op)
    }
}
