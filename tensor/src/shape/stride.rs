/*
   Appellation: stride <mod>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{Axis, Rank};
use core::borrow::{Borrow, BorrowMut};
use core::ops::{Deref, DerefMut, Index, IndexMut};
use core::slice::{Iter as SliceIter, IterMut as SliceIterMut};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub trait IntoStride {
    fn into_stride(self) -> Stride;
}

impl<S> IntoStride for S
where
    S: Into<Stride>,
{
    fn into_stride(self) -> Stride {
        self.into()
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Stride(pub(crate) Vec<usize>);

impl Stride {
    pub fn new(stride: Vec<usize>) -> Self {
        Self(stride)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    pub fn zeros(rank: Rank) -> Self {
        Self(vec![0; *rank])
    }
    /// Returns a reference to the stride.
    pub fn as_slice(&self) -> &[usize] {
        &self.0
    }
    /// Returns a mutable reference to the stride.
    pub fn as_slice_mut(&mut self) -> &mut [usize] {
        &mut self.0
    }
    /// Returns the capacity of the stride.
    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }
    /// Clears the stride, removing all elements.
    pub fn clear(&mut self) {
        self.0.clear()
    }
    /// Gets the element at the specified axis, returning None if the axis is out of bounds.
    pub fn get(&self, axis: Axis) -> Option<&usize> {
        self.0.get(*axis)
    }
    /// Returns an iterator over references to the elements of the stride.
    pub fn iter(&self) -> SliceIter<usize> {
        self.0.iter()
    }
    /// Returns an iterator over mutable references to the elements of the stride.
    pub fn iter_mut(&mut self) -> SliceIterMut<usize> {
        self.0.iter_mut()
    }
    /// Returns the rank of the stride; i.e., the number of dimensions.
    pub fn rank(&self) -> Rank {
        self.0.len().into()
    }
    /// Reverses the stride.
    pub fn reverse(&mut self) {
        self.0.reverse()
    }
    /// Swaps two elements in the stride, inplace.
    pub fn swap(&mut self, a: usize, b: usize) {
        self.0.swap(a, b)
    }
    /// Returns a new shape with the two axes swapped.
    pub fn swap_axes(&self, a: Axis, b: Axis) -> Self {
        let mut stride = self.clone();
        stride.swap(a.axis(), b.axis());
        stride
    }
}

// Internal methods
impl Stride {
    pub(crate) fn _fastest_varying_stride_order(&self) -> Self {
        let mut indices = self.clone();
        for (i, elt) in indices.as_slice_mut().into_iter().enumerate() {
            *elt = i;
        }
        let strides = self.as_slice();
        indices
            .as_slice_mut()
            .sort_by_key(|&i| (strides[i] as isize).abs());
        indices
    }
}

impl AsRef<[usize]> for Stride {
    fn as_ref(&self) -> &[usize] {
        &self.0
    }
}

impl AsMut<[usize]> for Stride {
    fn as_mut(&mut self) -> &mut [usize] {
        &mut self.0
    }
}

impl Borrow<[usize]> for Stride {
    fn borrow(&self) -> &[usize] {
        &self.0
    }
}

impl BorrowMut<[usize]> for Stride {
    fn borrow_mut(&mut self) -> &mut [usize] {
        &mut self.0
    }
}

impl Deref for Stride {
    type Target = [usize];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Stride {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Extend<usize> for Stride {
    fn extend<I: IntoIterator<Item = usize>>(&mut self, iter: I) {
        self.0.extend(iter)
    }
}

impl FromIterator<usize> for Stride {
    fn from_iter<I: IntoIterator<Item = usize>>(iter: I) -> Self {
        Stride(Vec::from_iter(iter))
    }
}

impl Index<usize> for Stride {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Stride {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Index<Axis> for Stride {
    type Output = usize;

    fn index(&self, index: Axis) -> &Self::Output {
        &self.0[*index]
    }
}

impl IndexMut<Axis> for Stride {
    fn index_mut(&mut self, index: Axis) -> &mut Self::Output {
        &mut self.0[*index]
    }
}

impl IntoIterator for Stride {
    type Item = usize;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl From<Vec<usize>> for Stride {
    fn from(v: Vec<usize>) -> Self {
        Stride(v)
    }
}

impl From<&[usize]> for Stride {
    fn from(v: &[usize]) -> Self {
        Stride(v.to_vec())
    }
}
