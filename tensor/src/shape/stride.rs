/*
   Appellation: stride <mod>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{Axis, Rank};
use core::borrow::{Borrow, BorrowMut};
use core::ops::{Deref, DerefMut};
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

    pub fn get(&self, index: usize) -> Option<&usize> {
        self.0.get(index)
    }

    pub fn iter(&self) -> std::slice::Iter<usize> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<usize> {
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
    /// Returns a reference to the stride.
    pub fn slice(&self) -> &[usize] {
        &self.0
    }
    /// Returns a mutable reference to the stride.
    pub fn slice_mut(&mut self) -> &mut [usize] {
        &mut self.0
    }
    /// Swaps two elements in the stride.
    pub fn swap(&mut self, a: usize, b: usize) {
        self.0.swap(a, b)
    }

    pub fn swap_axes(&self, a: Axis, b: Axis) -> Self {
        let mut stride = self.clone();
        stride.swap(a.axis(), b.axis());
        stride
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

// impl Iterator for Stride {
//     type Item = usize;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.0.next()
//     }
// }

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
