/*
    Appellation: layout <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Layout
//!
//!
use crate::shape::{IntoShape, Shape};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Layout {
    pub(crate) offset: usize,
    pub(crate) shape: Shape,
    pub(crate) stride: Vec<usize>,
}

impl Layout {
    pub fn new(offset: usize, shape: Shape, stride: Vec<usize>) -> Self {
        Self {
            offset,
            shape,
            stride,
        }
    }

    pub fn contiguous(shape: impl IntoShape) -> Self {
        let shape = shape.into_shape();
        let stride = shape.stride_contiguous();
        Self {
            offset: 0,
            shape,
            stride,
        }
    }

    pub fn contiguous_with_offset(shape: impl IntoShape, offset: usize) -> Self {
        let shape = shape.into_shape();
        let stride = shape.stride_contiguous();
        Self {
            offset,
            shape,
            stride,
        }
    }

    pub fn ndim(&self) -> usize {
        debug_assert_eq!(self.stride.len(), self.shape.ndim());
        self.shape.ndim()
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn shape(&self) -> &Shape {
        &self.shape
    }

    pub fn size(&self) -> usize {
        self.shape.size()
    }

    pub fn stride(&self) -> &[usize] {
        &self.stride
    }
}

// Internal methods
impl Layout {
    pub(crate) fn position(&self, coords: impl AsRef<[usize]>) -> usize {
        let mut index = self.offset;
        for (i, &coord) in coords.as_ref().iter().enumerate() {
            index += coord * self.stride[i];
        }
        index
    }
}
