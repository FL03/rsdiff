/*
    Appellation: layout <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::iter::LayoutIter;
use crate::shape::dim::stride_offset;
use crate::shape::{Axis, IntoShape, IntoStride, Rank, Shape, ShapeError, ShapeResult, Stride};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// The layout describes the memory layout of a tensor.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Layout {
    pub(crate) offset: usize,
    pub(crate) shape: Shape,
    pub(crate) strides: Stride,
}

impl Layout {
    pub unsafe fn new(offset: usize, shape: impl IntoShape, strides: impl IntoStride) -> Self {
        Self {
            offset,
            shape: shape.into_shape(),
            strides: strides.into_stride(),
        }
    }
    /// Create a new layout with a contiguous stride.
    pub fn contiguous(shape: impl IntoShape) -> Self {
        let shape = shape.into_shape();
        let stride = shape.stride_contiguous();
        Self {
            offset: 0,
            shape,
            strides: stride,
        }
    }
    #[doc(hidden)]
    /// Return stride offset for index.
    pub fn stride_offset(index: impl AsRef<[usize]>, strides: &Stride) -> isize {
        let mut offset = 0;
        for (&i, &s) in izip!(index.as_ref(), strides.as_slice()) {
            offset += stride_offset(i, s);
        }
        offset
    }
    /// Broadcast the layout to a new shape.
    ///
    /// The new shape must have the same or higher rank than the current shape.
    pub fn broadcast_as(&self, shape: impl IntoShape) -> ShapeResult<Self> {
        let shape = shape.into_shape();
        if shape.rank() < self.shape().rank() {
            return Err(ShapeError::IncompatibleShapes);
        }
        let diff = shape.rank() - self.shape().rank();
        let mut stride = vec![0; *diff];
        for (&dst_dim, (&src_dim, &src_stride)) in shape[*diff..]
            .iter()
            .zip(self.shape().iter().zip(self.strides().iter()))
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
        let layout = unsafe { Layout::new(0, shape, stride) };
        Ok(layout)
    }
    /// Determine if the current layout is contiguous or not.
    pub fn is_contiguous(&self) -> bool {
        self.shape().is_contiguous(&self.strides)
    }
    /// Checks to see if the layout is empy; i.e. a scalar of Rank(0)
    pub fn is_scalar(&self) -> bool {
        self.shape().is_scalar()
    }
    /// A function for determining if the layout is square.
    /// An n-dimensional object is square if all of its dimensions are equal.
    pub fn is_square(&self) -> bool {
        self.shape().is_square()
    }

    pub fn iter(&self) -> LayoutIter {
        LayoutIter::new(self.clone())
    }
    /// Peek the offset of the layout.
    pub fn offset(&self) -> usize {
        self.offset
    }
    /// Returns the offset from the lowest-address element to the logically first
    /// element.
    pub fn offset_from_low_addr_ptr_to_logical_ptr(&self) -> usize {
        let offset =
            izip!(self.shape().as_slice(), self.strides().as_slice()).fold(0, |acc, (d, s)| {
                let d = *d as isize;
                let s = *s as isize;
                if s < 0 && d > 1 {
                    acc - s * (d - 1)
                } else {
                    acc
                }
            });
        debug_assert!(offset >= 0);
        offset as usize
    }
    /// Return the rank (number of dimensions) of the layout.
    pub fn rank(&self) -> Rank {
        debug_assert_eq!(self.strides.len(), *self.shape.rank());
        self.shape.rank()
    }
    /// Remove an axis from the current layout, returning the new layout.
    pub fn remove_axis(&self, axis: Axis) -> Self {
        Self {
            offset: self.offset,
            shape: self.shape().remove_axis(axis),
            strides: self.strides().remove_axis(axis),
        }
    }
    /// Reshape the layout to a new shape.
    pub fn reshape(&mut self, shape: impl IntoShape) {
        self.shape = shape.into_shape();
        self.strides = self.shape.stride_contiguous();
    }
    /// Reverse the order of the axes.
    pub fn reverse(&mut self) {
        self.shape.reverse();
        self.strides.reverse();
    }
    /// Reverse the order of the axes.
    pub fn reverse_axes(mut self) -> Layout {
        self.reverse();
        self
    }
    /// Get a reference to the shape of the layout.
    pub const fn shape(&self) -> &Shape {
        &self.shape
    }
    /// Get a reference to the number of elements in the layout.
    pub fn size(&self) -> usize {
        self.shape().size()
    }
    /// Get a reference to the stride of the layout.
    pub const fn strides(&self) -> &Stride {
        &self.strides
    }
    /// Swap the axes of the layout.
    pub fn swap_axes(&self, a: Axis, b: Axis) -> Layout {
        Layout {
            offset: self.offset,
            shape: self.shape.swap_axes(a, b),
            strides: self.strides.swap_axes(a, b),
        }
    }
    /// Transpose the layout.
    pub fn transpose(&self) -> Layout {
        self.clone().reverse_axes()
    }

    pub fn with_offset(mut self, offset: usize) -> Self {
        self.offset = offset;
        self
    }

    pub fn with_shape_c(mut self, shape: impl IntoShape) -> Self {
        self.shape = shape.into_shape();
        self.strides = self.shape.stride_contiguous();
        self
    }

    pub unsafe fn with_shape_unchecked(mut self, shape: impl IntoShape) -> Self {
        self.shape = shape.into_shape();
        self
    }

    pub unsafe fn with_strides_unchecked(mut self, stride: impl IntoStride) -> Self {
        self.strides = stride.into_stride();
        self
    }
}

// Internal methods
impl Layout {
    pub(crate) fn index(&self, idx: impl AsRef<[usize]>) -> usize {
        let idx = idx.as_ref();
        debug_assert_eq!(idx.len(), *self.rank(), "Dimension mismatch");
        self.index_unchecked(idx)
    }

    pub(crate) fn index_unchecked(&self, idx: impl AsRef<[usize]>) -> usize {
        crate::coordinates_to_index(idx, self.strides())
    }
}

#[cfg(test)]
mod tests {
    use super::Layout;

    #[test]
    fn test_position() {
        let shape = (3, 3);
        let layout = Layout::contiguous(shape);
        assert_eq!(layout.index_unchecked([0, 0]), 0);
        assert_eq!(layout.index([0, 1]), 1);
        assert_eq!(layout.index([2, 2]), 8);
    }
}
