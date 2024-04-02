/*
    Appellation: reshape <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::ops::BoxTensor;
use crate::shape::Shape;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumDiscriminants, EnumIs, EnumIter, EnumString, VariantNames};

#[derive(Clone, Debug, EnumDiscriminants, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
#[strum(serialize_all = "snake_case")]
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
#[cfg_attr(feature = "serde", strum_discriminants(derive(Deserialize, Serialize)))]
#[strum_discriminants(name(ReshapeOp))]
pub enum ReshapeExpr<T> {
    Broadcast { scope: BoxTensor<T>, shape: Shape },
    Reshape { scope: BoxTensor<T>, shape: Shape },
    Swap,
    Transpose,
}
