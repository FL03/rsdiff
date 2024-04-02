/*
    Appellation: layout <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::shape::{Axis, IntoShape, IntoStride, Rank, Shape, ShapeError, ShapeResult, Stride};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// The layout describes the memory layout of a tensor.
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
        let diff = shape.rank() - self.shape().rank();
        let mut stride = vec![0; *diff];
        for (&dst_dim, (&src_dim, &src_stride)) in shape[*diff..]
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
    /// Determine if the current layout is contiguous or not.
    pub fn is_contiguous(&self) -> bool {
        self.shape.is_contiguous(&self.stride)
    }
    pub fn is_layout_c(&self) -> bool {
        if let 1 = *self.shape.rank() {
            return self.stride[0] == 1 || self.shape[0] <= 1;
        }

        for d in self.shape().iter() {
            if *d == 0 {
                return true;
            }
        }

        let mut contig_stride = 1_isize;
        // check all dimensions -- a dimension of length 1 can have unequal strides
        for (dim, s) in izip!(self.shape().iter().rev(), self.stride().iter().rev()) {
            if *dim != 1 {
                let s = *s as isize;
                if s != contig_stride {
                    return false;
                }
                contig_stride *= *dim as isize;
            }
        }
        true
    }
    /// Determine if the current layout is square or not.
    pub fn is_square(&self) -> bool {
        self.shape.is_square()
    }
    /// Get a peek at the offset of the layout.
    pub fn offset(&self) -> usize {
        self.offset
    }
    /// Returns the offset from the lowest-address element to the logically first
    /// element.
    pub fn offset_from_low_addr_ptr_to_logical_ptr(&self) -> usize {
        let offset =
            izip!(self.shape().slice(), self.stride().slice()).fold(0, |_offset, (d, s)| {
                let d = *d as isize;
                let s = *s as isize;
                if s < 0 && d > 1 {
                    _offset - s * (d - 1)
                } else {
                    _offset
                }
            });
        debug_assert!(offset >= 0);
        offset as usize
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
    /// Get a reference to the shape of the layout.
    pub const fn shape(&self) -> &Shape {
        &self.shape
    }
    /// Get a reference to the number of elements in the layout.
    pub fn size(&self) -> usize {
        self.shape.size()
    }
    /// Get a reference to the stride of the layout.
    pub const fn stride(&self) -> &Stride {
        &self.stride
    }

    pub fn swap_axes(&self, a: Axis, b: Axis) -> Layout {
        Layout {
            offset: self.offset,
            shape: self.shape.swap_axes(a, b),
            stride: self.stride.swap_axes(a, b),
        }
    }

    pub fn transpose(&self) -> Layout {
        let mut layout = self.clone();
        layout.shape.reverse();
        layout.stride.reverse();
        layout
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
#[allow(dead_code)]
impl Layout {
    pub(crate) fn index(&self, idx: impl AsRef<[usize]>) -> usize {
        let idx = idx.as_ref();
        if idx.len() != *self.shape.rank() {
            panic!("Dimension mismatch");
        }
        idx.iter().zip(self.stride.iter()).map(|(i, s)| i * s).sum()
    }

    pub(crate) fn index_unchecked(&self, idx: impl AsRef<[usize]>) -> usize {
        idx.as_ref()
            .iter()
            .zip(self.stride.iter())
            .map(|(i, s)| i * s)
            .sum()
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
