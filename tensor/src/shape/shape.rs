/*
   Appellation: shape <mod>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::error::ShapeError;
use super::{Axis, Rank, Stride};
use crate::prelude::TensorResult;

use core::ops::{self, Deref};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize,))]
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Shape(Vec<usize>);

impl Shape {
    pub fn new(shape: Vec<usize>) -> Self {
        Self(shape)
    }

    pub fn scalar() -> Self {
        Self(Vec::new())
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    pub fn zeros(rank: usize) -> Self {
        Self(vec![0; rank])
    }

    pub(crate) fn matmul_shape(&self, other: &Self) -> TensorResult<Self> {
        if *self.rank() != 2 || *other.rank() != 2 || self[1] != other[0] {
            return Err(ShapeError::IncompatibleShapes.into());
        }
        Ok(Self::from((self[0], other[1])))
    }

    pub fn dims(&self) -> &[usize] {
        &self.0
    }

    pub fn insert(&mut self, index: usize, dim: usize) {
        self.0.insert(index, dim)
    }

    /// Returns true if the strides are C contiguous (aka row major).
    pub fn is_contiguous(&self, stride: &Stride) -> bool {
        if self.0.len() != stride.len() {
            return false;
        }
        let mut acc = 1;
        for (&stride, &dim) in stride.iter().zip(self.0.iter()).rev() {
            if stride != acc {
                return false;
            }
            acc *= dim;
        }
        true
    }

    pub fn ncols(&self) -> usize {
        if self.len() >= 2 {
            self.0[1]
        } else if self.len() == 1 {
            1
        } else {
            0
        }
    }

    pub fn nrows(&self) -> usize {
        if self.len() >= 1 {
            *self.0.first().unwrap()
        } else {
            0
        }
    }

    pub fn push(&mut self, dim: usize) {
        self.0.push(dim)
    }

    pub fn rank(&self) -> Rank {
        self.0.len().into()
    }

    pub fn remove(&mut self, index: usize) -> usize {
        self.0.remove(index)
    }

    pub fn reverse(&mut self) {
        self.0.reverse()
    }

    pub fn set(&mut self, index: Axis, dim: usize) {
        self[index] = dim
    }

    pub fn size(&self) -> usize {
        self.0.iter().product()
    }

    pub fn slice(&self) -> &[usize] {
        &self.0
    }

    pub fn slice_mut(&mut self) -> &mut [usize] {
        &mut self.0
    }

    pub(crate) fn swap(&mut self, a: Axis, b: Axis) {
        self.0.swap(a.axis(), b.axis())
    }

    pub fn swap_axes(&self, swap: Axis, with: Axis) -> Self {
        let mut shape = self.clone();
        shape.swap(swap, with);
        shape
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

    pub fn upcast(&self, to: &Shape, stride: &Stride) -> Option<Stride> {
        let mut new_stride = to.slice().to_vec();
        // begin at the back (the least significant dimension)
        // size of the axis has to either agree or `from` has to be 1
        if to.rank() < self.rank() {
            return None;
        }

        {
            let mut new_stride_iter = new_stride.as_mut_slice().iter_mut().rev();
            for ((er, es), dr) in self
                .slice()
                .iter()
                .rev()
                .zip(stride.slice().iter().rev())
                .zip(new_stride_iter.by_ref())
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
            for dr in new_stride_iter {
                *dr = 0;
            }
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

impl FromIterator<usize> for Shape {
    fn from_iter<I: IntoIterator<Item = usize>>(iter: I) -> Self {
        Self(Vec::from_iter(iter))
    }
}

impl IntoIterator for Shape {
    type Item = usize;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a mut Shape {
    type Item = &'a mut usize;
    type IntoIter = std::slice::IterMut<'a, usize>;

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
