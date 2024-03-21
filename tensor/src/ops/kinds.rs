/*
    Appellation: kinds <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::TensorBase;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIs, EnumIter, EnumString, VariantNames};

#[derive(Clone, Debug)]
pub enum Op<T> {
    Binary(Box<TensorBase<T>>, Box<TensorBase<T>>, BinaryOp),
    Unary(Box<TensorBase<T>>, UnaryOp),
}

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
    VariantNames,
)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "lowercase", untagged)
)]
#[repr(u8)]
#[strum(serialize_all = "lowercase")]
pub enum BinaryOp {
    Add,
    Div,
    Matmul,
    Mul,
    Sub,
}

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
    VariantNames,
)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "lowercase", untagged)
)]
#[repr(u8)]
#[strum(serialize_all = "lowercase")]
pub enum UnaryOp {
    Abs,
    Cos,
    Cosh,
    Exp,
    Log,
    Ln,
    Neg,
    Reciprocal,
    Sin,
    Sinh,
    Sqrt,
    Square,
    Tan,
    Tanh,
}

pub struct BinOp<T> {
    pub lhs: TensorBase<T>,
    pub rhs: TensorBase<T>,
    pub op: BinaryOp,
}
