/*
   Appellation: shape <mod>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{Axis, Rank, ShapeError, Stride};
use crate::prelude::{SwapAxes, TensorResult};
#[cfg(not(feature = "std"))]
use alloc::vec;
use core::ops::{self, Deref};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "std")]
use std::vec;
/// A shape is a description of the number of elements in each dimension.
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize,))]
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Shape(Vec<usize>);

impl Shape {
    pub fn new(shape: Vec<usize>) -> Self {
        Self(shape)
    }
    /// Creates a new shape of rank 0.
    pub fn scalar() -> Self {
        Self(Vec::new())
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }
    /// Creates a new shape of the given rank with all dimensions set to 0.
    pub fn zeros(rank: usize) -> Self {
        Self(vec![0; rank])
    }
    /// Get a reference to the shape as a slice.
    pub fn as_slice(&self) -> &[usize] {
        &self.0
    }
    /// Get a mutable reference to the shape as a slice.
    pub fn as_slice_mut(&mut self) -> &mut [usize] {
        &mut self.0
    }

    pub fn check_size(&self) -> Result<usize, ShapeError> {
        let size_nonzero = self
            .as_slice()
            .iter()
            .filter(|&&d| d != 0)
            .try_fold(1usize, |acc, &d| acc.checked_mul(d))
            .ok_or_else(|| ShapeError::Overflow)?;
        if size_nonzero > ::std::isize::MAX as usize {
            Err(ShapeError::Overflow)
        } else {
            Ok(self.size())
        }
    }
    /// Attempts to create a one-dimensional shape that describes the
    /// diagonal of the current shape.
    pub fn diag(&self) -> Shape {
        Self::new(i![self.nrows()])
    }
    pub fn get_final_position(&self) -> Vec<usize> {
        self.iter().map(|&dim| dim - 1).collect()
    }
    /// Inserts a new dimension along the given [Axis].
    pub fn insert(&mut self, index: Axis, dim: usize) {
        self.0.insert(*index, dim)
    }
    /// Inserts a new dimension along the given [Axis].
    pub fn insert_axis(&self, index: Axis) -> Self {
        let mut shape = self.clone();
        shape.insert(index, 1);
        shape
    }
    /// Returns true if the shape is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    /// Returns true if the shape is a scalar.
    pub fn is_scalar(&self) -> bool {
        self.is_empty()
    }
    /// Checks to see if the shape is square
    pub fn is_square(&self) -> bool {
        self.iter().all(|&dim| dim == self[0])
    }
    /// Returns true if the strides are C contiguous (aka row major).
    pub fn is_contiguous(&self, stride: &Stride) -> bool {
        if self.0.len() != stride.len() {
            return false;
        }
        let mut acc = 1;
        for (&stride, &dim) in stride.iter().zip(self.iter()).rev() {
            if stride != acc {
                return false;
            }
            acc *= dim;
        }
        true
    }
    /// The number of columns in the shape.
    pub fn ncols(&self) -> usize {
        if self.len() >= 2 {
            self[1]
        } else if self.len() == 1 {
            1
        } else {
            0
        }
    }
    /// The number of rows in the shape.
    pub fn nrows(&self) -> usize {
        if self.len() >= 1 {
            self[0]
        } else {
            0
        }
    }
    /// Removes and returns the last dimension of the shape.
    pub fn pop(&mut self) -> Option<usize> {
        self.0.pop()
    }
    /// Add a new dimension to the shape.
    pub fn push(&mut self, dim: usize) {
        self.0.push(dim)
    }
    /// Get the number of dimensions, or [Rank], of the shape
    pub fn rank(&self) -> Rank {
        self.0.len().into()
    }
    /// Remove the dimension at the given [Axis],
    pub fn remove(&mut self, index: Axis) -> usize {
        self.0.remove(*index)
    }
    /// Remove the dimension at the given [Axis].
    pub fn remove_axis(&self, index: Axis) -> Shape {
        let mut shape = self.clone();
        shape.remove(index);
        shape
    }
    /// Reverse the dimensions of the shape.
    pub fn reverse(&mut self) {
        self.0.reverse()
    }
    /// Set the dimension at the given [Axis].
    pub fn set(&mut self, index: Axis, dim: usize) {
        self[index] = dim
    }
    /// The number of elements in the shape.
    pub fn size(&self) -> usize {
        self.0.iter().product()
    }

    /// Swap the dimensions of the current [Shape] at the given [Axis].
    pub fn swap(&mut self, a: Axis, b: Axis) {
        self.0.swap(a.axis(), b.axis())
    }
    /// Swap the dimensions at the given [Axis], creating a new [Shape]
    pub fn swap_axes(&self, swap: Axis, with: Axis) -> Self {
        let mut shape = self.clone();
        shape.swap(swap, with);
        shape
    }
}

// Internal methods
#[allow(dead_code)]
#[doc(hidden)]
impl Shape {
    pub(crate) fn default_strides(&self) -> Stride {
        // Compute default array strides
        // Shape (a, b, c) => Give strides (b * c, c, 1)
        let mut strides = Stride::zeros(self.rank());
        // For empty arrays, use all zero strides.
        if self.iter().all(|&d| d != 0) {
            let mut it = strides.as_slice_mut().iter_mut().rev();
            // Set first element to 1
            if let Some(rs) = it.next() {
                *rs = 1;
            }
            let mut cum_prod = 1;
            for (rs, dim) in it.zip(self.iter().rev()) {
                cum_prod *= *dim;
                *rs = cum_prod;
            }
        }
        strides
    }

    pub(crate) fn matmul_shape(&self, other: &Self) -> TensorResult<Self> {
        if *self.rank() != 2 || *other.rank() != 2 || self[1] != other[0] {
            return Err(ShapeError::IncompatibleShapes.into());
        }
        Ok(Self::from((self[0], other[1])))
    }

    pub(crate) fn stride_contiguous(&self) -> Stride {
        let mut stride: Vec<_> = self
            .0
            .iter()
            .rev()
            .scan(1, |prod, u| {
                let prod_pre_mult = *prod;
                *prod *= u;
                Some(prod_pre_mult)
            })
            .collect();
        stride.reverse();
        stride.into()
    }

    pub(crate) fn upcast(&self, to: &Shape, stride: &Stride) -> Option<Stride> {
        let mut new_stride = to.as_slice().to_vec();
        // begin at the back (the least significant dimension)
        // size of the axis has to either agree or `from` has to be 1
        if to.rank() < self.rank() {
            return None;
        }

        let mut iter = new_stride.as_mut_slice().iter_mut().rev();
        for ((er, es), dr) in self
            .as_slice()
            .iter()
            .rev()
            .zip(stride.as_slice().iter().rev())
            .zip(iter.by_ref())
        {
            /* update strides */
            if *dr == *er {
                /* keep stride */
                *dr = *es;
            } else if *er == 1 {
                /* dead dimension, zero stride */
                *dr = 0
            } else {
                return None;
            }
        }

        /* set remaining strides to zero */
        for dr in iter {
            *dr = 0;
        }

        Some(new_stride.into())
    }
}

impl AsRef<[usize]> for Shape {
    fn as_ref(&self) -> &[usize] {
        &self.0
    }
}

impl AsMut<[usize]> for Shape {
    fn as_mut(&mut self) -> &mut [usize] {
        &mut self.0
    }
}

impl Deref for Shape {
    type Target = [usize];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Extend<usize> for Shape {
    fn extend<I: IntoIterator<Item = usize>>(&mut self, iter: I) {
        self.0.extend(iter)
    }
}

impl SwapAxes for Shape {
    fn swap_axes(&self, a: Axis, b: Axis) -> Self {
        self.swap_axes(a, b)
    }
}

impl FromIterator<usize> for Shape {
    fn from_iter<I: IntoIterator<Item = usize>>(iter: I) -> Self {
        Self(Vec::from_iter(iter))
    }
}

impl IntoIterator for Shape {
    type Item = usize;
    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a mut Shape {
    type Item = &'a mut usize;
    type IntoIter = core::slice::IterMut<'a, usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl ops::Index<usize> for Shape {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl ops::Index<Axis> for Shape {
    type Output = usize;

    fn index(&self, index: Axis) -> &Self::Output {
        &self.0[*index]
    }
}

impl ops::IndexMut<usize> for Shape {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl ops::IndexMut<Axis> for Shape {
    fn index_mut(&mut self, index: Axis) -> &mut Self::Output {
        &mut self.0[*index]
    }
}

impl ops::Index<ops::Range<usize>> for Shape {
    type Output = [usize];

    fn index(&self, index: ops::Range<usize>) -> &Self::Output {
        &self.0[index]
    }
}

impl ops::Index<ops::RangeTo<usize>> for Shape {
    type Output = [usize];

    fn index(&self, index: ops::RangeTo<usize>) -> &Self::Output {
        &self.0[index]
    }
}

impl ops::Index<ops::RangeFrom<usize>> for Shape {
    type Output = [usize];

    fn index(&self, index: ops::RangeFrom<usize>) -> &Self::Output {
        &self.0[index]
    }
}

impl ops::Index<ops::RangeFull> for Shape {
    type Output = [usize];

    fn index(&self, index: ops::RangeFull) -> &Self::Output {
        &self.0[index]
    }
}

impl ops::Index<ops::RangeInclusive<usize>> for Shape {
    type Output = [usize];

    fn index(&self, index: ops::RangeInclusive<usize>) -> &Self::Output {
        &self.0[index]
    }
}

impl ops::Index<ops::RangeToInclusive<usize>> for Shape {
    type Output = [usize];

    fn index(&self, index: ops::RangeToInclusive<usize>) -> &Self::Output {
        &self.0[index]
    }
}

unsafe impl Send for Shape {}

unsafe impl Sync for Shape {}

impl From<()> for Shape {
    fn from(_: ()) -> Self {
        Self::default()
    }
}

impl From<usize> for Shape {
    fn from(dim: usize) -> Self {
        Self(vec![dim])
    }
}

impl From<Vec<usize>> for Shape {
    fn from(shape: Vec<usize>) -> Self {
        Self(shape)
    }
}

impl From<&[usize]> for Shape {
    fn from(shape: &[usize]) -> Self {
        Self(shape.to_vec())
    }
}

impl From<(usize,)> for Shape {
    fn from(shape: (usize,)) -> Self {
        Self(vec![shape.0])
    }
}

impl From<(usize, usize)> for Shape {
    fn from(shape: (usize, usize)) -> Self {
        Self(vec![shape.0, shape.1])
    }
}

impl From<(usize, usize, usize)> for Shape {
    fn from(shape: (usize, usize, usize)) -> Self {
        Self(vec![shape.0, shape.1, shape.2])
    }
}

impl From<(usize, usize, usize, usize)> for Shape {
    fn from(shape: (usize, usize, usize, usize)) -> Self {
        Self(vec![shape.0, shape.1, shape.2, shape.3])
    }
}

impl From<(usize, usize, usize, usize, usize)> for Shape {
    fn from(shape: (usize, usize, usize, usize, usize)) -> Self {
        Self(vec![shape.0, shape.1, shape.2, shape.3, shape.4])
    }
}

impl From<(usize, usize, usize, usize, usize, usize)> for Shape {
    fn from(shape: (usize, usize, usize, usize, usize, usize)) -> Self {
        Self(vec![shape.0, shape.1, shape.2, shape.3, shape.4, shape.5])
    }
}

// macro_rules! tuple_vec {
//     ($($n:tt),*) => {
//         vec![$($n,)*]
//     };

// }

// macro_rules! impl_from_tuple {
//     ($($n:tt: $name:ident),+) => {
//         impl<$($name),+> From<($($name,)+)> for Shape
//         where
//             $($name: Into<usize>,)+
//         {
//             fn from(shape: ($($name,)+)) -> Self {
//                 Self(vec![$($name.into(),)+])
//             }
//         }
//     };
// }

// impl_from_tuple!(A: A);
