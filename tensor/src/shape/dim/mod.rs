/*
   Appellation: dim <mod>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Dimension
//!

pub use self::{dimension::Dim, utils::*};

pub(crate) mod dimension;

use super::Rank;

pub trait IntoDimension {
    type Dim: Dimension;

    fn into_dimension(self) -> Self::Dim;
}

pub trait Dimension {
    type Pattern;

    fn as_slice(&self) -> &[usize];
    /// Return the [rank](super::Rank) of the dimension; i.e. the number of axes.
    fn rank(&self) -> Rank;
    /// Return the size of the dimension; i.e. the number of elements.
    fn size(&self) -> usize;

    #[doc(hidden)]
    /// Return stride offset for index.
    fn stride_offset(index: &Self, strides: &Self) -> isize {
        izip!(index.as_slice(), strides.as_slice())
            .fold(0, |acc, (&i, &s)| acc + stride_offset(i, s))
    }
}

pub(crate) mod utils {
    use crate::index::{Ix, Ixs};
    use crate::shape::{Shape, ShapeError, Stride};
    use core::mem;

    pub fn can_index_slice<A>(
        data: &[A],
        shape: &Shape,
        stride: &Stride,
    ) -> Result<(), ShapeError> {
        // Check conditions 1 and 2 and calculate `max_offset`.
        let max_offset = max_abs_offset_check_overflow::<A>(shape, stride)?;
        can_index_slice_impl(max_offset, data.len(), shape, stride)
    }

    pub fn dim_stride_overlap(dim: &Shape, strides: &Stride) -> bool {
        let order = strides._fastest_varying_stride_order();
        let mut sum_prev_offsets = 0;
        for &index in order.as_slice() {
            let d = dim[index];
            let s = (strides[index] as isize).abs();
            match d {
                0 => return false,
                1 => {}
                _ => {
                    if s <= sum_prev_offsets {
                        return true;
                    }
                    sum_prev_offsets += (d - 1) as isize * s;
                }
            }
        }
        false
    }

    pub fn max_abs_offset_check_overflow<A>(
        dim: &Shape,
        strides: &Stride,
    ) -> Result<usize, ShapeError> {
        max_abs_offset_check_overflow_impl(mem::size_of::<A>(), dim, strides)
    }

    pub fn size_of_shape_checked(dim: &Shape) -> Result<usize, ShapeError> {
        let size_nonzero = dim
            .as_slice()
            .iter()
            .filter(|&&d| d != 0)
            .try_fold(1usize, |acc, &d| acc.checked_mul(d))
            .ok_or(ShapeError::Overflow)?;
        if size_nonzero > ::std::isize::MAX as usize {
            Err(ShapeError::Overflow)
        } else {
            Ok(dim.size())
        }
    }

    /// Calculate offset from `Ix` stride converting sign properly
    #[inline(always)]
    pub fn stride_offset(n: Ix, stride: Ix) -> isize {
        (n as isize) * (stride as Ixs)
    }

    fn can_index_slice_impl(
        max_offset: usize,
        data_len: usize,
        dim: &Shape,
        strides: &Stride,
    ) -> Result<(), ShapeError> {
        // Check condition 3.
        let is_empty = dim.as_slice().iter().any(|&d| d == 0);
        if is_empty && max_offset > data_len {
            return Err(ShapeError::OutOfBounds);
        }
        if !is_empty && max_offset >= data_len {
            return Err(ShapeError::OutOfBounds);
        }

        // Check condition 4.
        if !is_empty && dim_stride_overlap(dim, strides) {
            return Err(ShapeError::Unsupported);
        }

        Ok(())
    }

    fn max_abs_offset_check_overflow_impl(
        elem_size: usize,
        dim: &Shape,
        strides: &Stride,
    ) -> Result<usize, ShapeError> {
        // Condition 1.
        if dim.rank() != strides.rank() {
            return Err(ShapeError::IncompatibleLayout);
        }

        // Condition 3.
        let _ = size_of_shape_checked(dim)?;

        // Determine absolute difference in units of `A` between least and greatest
        // address accessible by moving along all axes.
        let max_offset: usize = izip!(dim.as_slice(), strides.as_slice())
            .try_fold(0usize, |acc, (&d, &s)| {
                let s = s as isize;
                // Calculate maximum possible absolute movement along this axis.
                let off = d.saturating_sub(1).checked_mul(s.unsigned_abs())?;
                acc.checked_add(off)
            })
            .ok_or_else(|| ShapeError::Overflow)?;
        // Condition 2a.
        if max_offset > isize::MAX as usize {
            return Err(ShapeError::Overflow);
        }

        // Determine absolute difference in units of bytes between least and
        // greatest address accessible by moving along all axes
        let max_offset_bytes = max_offset
            .checked_mul(elem_size)
            .ok_or(ShapeError::Overflow)?;
        // Condition 2b.
        if max_offset_bytes > isize::MAX as usize {
            return Err(ShapeError::Overflow);
        }

        Ok(max_offset)
    }
}
