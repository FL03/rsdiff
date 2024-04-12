/*
    Appellation: reshape <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::ops::{BoxTensor, TensorExpr};
use crate::shape::{Axis, Shape};
use crate::tensor::TensorBase;
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
    Broadcast {
        recv: BoxTensor<T>,
        shape: Shape,
    },
    Reshape {
        recv: BoxTensor<T>,
        shape: Shape,
    },
    Swap {
        recv: BoxTensor<T>,
        a: usize,
        b: usize,
    },
    SwapAxis {
        recv: BoxTensor<T>,
        a: Axis,
        b: Axis,
    },
    Transpose {
        recv: BoxTensor<T>,
    },
}

impl<T> ReshapeExpr<T> {
    pub fn broadcast(recv: TensorBase<T>, shape: Shape) -> Self {
        Self::Broadcast {
            recv: recv.boxed(),
            shape,
        }
    }

    pub fn reshape(recv: TensorBase<T>, shape: Shape) -> Self {
        Self::Reshape {
            recv: recv.boxed(),
            shape,
        }
    }

    pub fn swap(recv: TensorBase<T>, a: usize, b: usize) -> Self {
        Self::Swap {
            recv: recv.boxed(),
            a,
            b,
        }
    }

    pub fn swap_axes(recv: TensorBase<T>, a: Axis, b: Axis) -> Self {
        Self::SwapAxis {
            recv: recv.boxed(),
            a,
            b,
        }
    }

    pub fn transpose(recv: TensorBase<T>) -> Self {
        Self::Transpose { recv: recv.boxed() }
    }

    pub fn recv(&self) -> &BoxTensor<T> {
        match self {
            Self::Broadcast { recv, .. } => recv,
            Self::Reshape { recv, .. } => recv,
            Self::Swap { recv, .. } => recv,
            Self::SwapAxis { recv, .. } => recv,
            Self::Transpose { recv } => recv,
        }
    }

    pub fn recv_mut(&mut self) -> &mut BoxTensor<T> {
        match self {
            Self::Broadcast { recv, .. } => recv,
            Self::Reshape { recv, .. } => recv,
            Self::Swap { recv, .. } => recv,
            Self::SwapAxis { recv, .. } => recv,
            Self::Transpose { recv } => recv,
        }
    }

    pub fn view(&self) -> ReshapeExpr<&T> {
        match self {
            Self::Broadcast { recv, shape } => ReshapeExpr::broadcast(recv.view(), shape.clone()),
            Self::Reshape { recv, shape } => ReshapeExpr::reshape(recv.view(), shape.clone()),
            Self::Swap { recv, a, b } => ReshapeExpr::swap(recv.view(), *a, *b),
            Self::SwapAxis { recv, a, b } => ReshapeExpr::swap_axes(recv.view(), *a, *b),
            Self::Transpose { recv } => ReshapeExpr::transpose(recv.view()),
        }
    }

    pub fn view_mut(&mut self) -> ReshapeExpr<&mut T> {
        match self {
            Self::Broadcast { recv, shape } => {
                ReshapeExpr::broadcast(recv.view_mut(), shape.clone())
            }
            Self::Reshape { recv, shape } => ReshapeExpr::reshape(recv.view_mut(), shape.clone()),
            Self::Swap { recv, a, b } => ReshapeExpr::swap(recv.view_mut(), *a, *b),
            Self::SwapAxis { recv, a, b } => ReshapeExpr::swap_axes(recv.view_mut(), *a, *b),
            Self::Transpose { recv } => ReshapeExpr::transpose(recv.view_mut()),
        }
    }
}

impl<A, B> From<ReshapeExpr<A>> for TensorExpr<A, B> {
    fn from(expr: ReshapeExpr<A>) -> Self {
        TensorExpr::Shape(expr)
    }
}
