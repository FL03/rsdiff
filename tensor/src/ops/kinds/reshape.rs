/*
    Appellation: reshape <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::ops::BoxTensor;
use crate::shape::Shape;
use strum::{Display, EnumCount, EnumDiscriminants, EnumIs, EnumIter, EnumString, VariantNames};

#[derive(Clone, Debug, EnumDiscriminants, Eq, Hash, PartialEq)]
#[repr(C)]
#[strum_discriminants(derive(
    Display,
    EnumCount,
    EnumIs,
    EnumIter,
    EnumString,
    Hash,
    Ord,
    PartialOrd,
    VariantNames
))]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "snake_case"),
    strum(serialize_all = "snake_case"),
    strum_discriminants(derive(serde::Deserialize, serde::Serialize))
)]
#[strum_discriminants(name(ReshapeOp))]
pub enum ReshapeExpr<T> {
    Broadcast { scope: BoxTensor<T>, shape: Shape },
    Reshape { scope: BoxTensor<T>, shape: Shape },
    Swap,
    Transpose,
}
