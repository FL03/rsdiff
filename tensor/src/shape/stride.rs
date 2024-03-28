/*
   Appellation: stride <mod>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
use core::ops::{Deref, DerefMut};

pub trait IntoStride {
    fn into_stride(self) -> Stride;
}

pub struct Stride(pub Vec<usize>);

impl Stride {
    pub fn ndim(&self) -> usize {
        self.0.len()
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

impl IntoIterator for Stride {
    type Item = usize;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
