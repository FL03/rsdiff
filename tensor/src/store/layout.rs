/*
    Appellation: layout <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::shape::{Axis, IntoShape, IntoStride, Rank, Shape, ShapeError, ShapeResult, Stride};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A layout is a description of how data is stored in memory.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Layout {
    pub(crate) offset: usize,
    pub(crate) shape: Shape,
    pub(crate) stride: Stride,
}

impl Layout {
    pub fn new(offset: usize, shape: impl IntoShape, stride: impl IntoStride) -> Self {
        Self {
            offset,
            shape: shape.into_shape(),
            stride: stride.into_stride(),
        }
    }
    /// Broadcast the layout to a new shape.
    ///
    /// The new shape must have the same or higher rank than the current shape.
    pub fn broadcast_as(&self, shape: impl IntoShape) -> ShapeResult<Self> {
        let shape = shape.into_shape();
        if shape.rank() < self.shape().rank() {
            return Err(ShapeError::IncompatibleShapes);
        }
        let added_dims = shape.rank() - self.shape().rank();
        let mut stride = vec![0; added_dims];
        for (&dst_dim, (&src_dim, &src_stride)) in shape[added_dims..]
            .iter()
            .zip(self.shape().iter().zip(self.stride().iter()))
        {
            let s = if dst_dim == src_dim {
                src_stride
            } else if src_dim != 1 {
                return Err(ShapeError::IncompatibleShapes);
            } else {
                0
            };
            stride.push(s)
        }
        Ok(Self::new(self.offset, shape, stride))
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
    /// Get a peek at the offset of the layout.
    pub fn offset(&self) -> usize {
        self.offset
    }
    /// Return the rank (number of dimensions) of the layout.
    pub fn rank(&self) -> Rank {
        debug_assert_eq!(self.stride.len(), *self.shape.rank());
        self.shape.rank()
    }
    /// Reshape the layout to a new shape.
    pub fn reshape(&mut self, shape: impl IntoShape) {
        self.shape = shape.into_shape();
        self.stride = self.shape.stride_contiguous();
    }
    /// Reverse the order of the axes.
    pub fn reverse_axes(mut self) -> Layout {
        self.shape.reverse();
        self.stride.reverse();
        self
    }

    pub fn shape(&self) -> Shape {
        self.shape.clone()
    }

    pub fn size(&self) -> usize {
        self.shape.size()
    }

    pub fn stride(&self) -> &Stride {
        &self.stride
    }

    pub fn swap_axes(&self, a: Axis, b: Axis) -> Layout {
        Layout {
            offset: self.offset,
            shape: self.shape.swap_axes(a, b),
            stride: self.stride.swap_axes(a, b),
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
    pub(crate) fn index(&self, coords: impl AsRef<[usize]>) -> usize {
        let coords = coords.as_ref();
        if coords.len() != *self.shape.rank() {
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
        assert_eq!(layout.index(&[0, 0]), 0);
        assert_eq!(layout.index(&[0, 1]), 1);
        assert_eq!(layout.index(&[2, 2]), 8);
    }
}
