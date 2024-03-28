/*
    Appellation: layout <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Layout
//!
//!
use crate::shape::{Axis, IntoShape, Shape, ShapeError, ShapeResult};
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
    /// Create a new layout with a contiguous stride.
    pub fn contiguous(shape: impl IntoShape) -> Self {
        let shape = shape.into_shape();
        let stride = shape.stride_contiguous();
        Self {
            offset: 0,
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

    pub fn reshape(&mut self, shape: impl IntoShape) {
        self.shape = shape.into_shape();
        self.stride = self.shape.stride_contiguous();
    }

    pub fn reverse_axes(mut self) -> Layout {
        self.shape.slice_mut().reverse();
        self.stride.reverse();
        self
    }

    pub fn shape(&self) -> Shape {
        self.shape.clone()
    }

    pub fn size(&self) -> usize {
        self.shape.size()
    }

    pub fn stride(&self) -> &[usize] {
        &self.stride
    }

    pub fn swap_axes(&self, a: Axis, b: Axis) -> Layout {
        let mut stride = self.stride.to_vec();
        stride.swap(a.axis(), b.axis());
        Layout {
            offset: self.offset,
            shape: self.shape.swap_axes(a, b),
            stride,
        }
    }

    pub fn transpose(&self, a: Axis, b: Axis) -> Layout {
        let shape = self.shape.swap_axes(a, b);
        let stride = shape.stride_contiguous();
        Layout {
            offset: self.offset,
            shape,
            stride,
        }
    }

    pub fn with_offset(mut self, offset: usize) -> Self {
        self.offset = offset;
        self
    }

    pub fn with_shape(mut self, shape: impl IntoShape) -> Self {
        self.shape = shape.into_shape();
        self.stride = self.shape.stride_contiguous();
        self
    }
}

// Internal methods
impl Layout {
    pub fn position(&self, coords: impl AsRef<[usize]>) -> ShapeResult<usize> {
        let coords = coords.as_ref();
        if coords.len() != self.shape.ndim() {
            return Err(ShapeError::DimensionMismatch.into());
        }
        for (&coord, &dim) in coords.iter().zip(self.shape.slice().iter()) {
            if coord >= dim {
                return Err(ShapeError::MismatchedElements.into());
            }
        }
        let mut index = self.offset;
        for (i, &coord) in coords.iter().enumerate() {
            index += coord * self.stride[i];
        }
        Ok(index)
    }

    pub fn select(&self, coords: impl AsRef<[usize]>) -> usize {
        let coords = coords.as_ref();
        if coords.len() != self.shape.ndim() {
            panic!("Dimension mismatch");
        }
        let index = coords
            .iter()
            .zip(self.stride.iter())
            .fold(self.offset, |acc, (&coord, &stride)| acc + coord * stride);
        index
    }
}

#[cfg(test)]
mod tests {
    use super::Layout;

    #[test]
    fn test_position() {
        let shape = (3, 3);
        let layout = Layout::contiguous(shape);
        assert_eq!(layout.select(&[0, 0]), 0);
        assert_eq!(layout.select(&[0, 1]), 1);
        assert_eq!(layout.select(&[2, 2]), 8);
    }
}
