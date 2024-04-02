/*
    Appellation: stride <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::Strided;
use crate::tensor::TensorBase;

pub struct IndexedIter<'a, T> {
    scope: Option<&'a T>,
    strides: Strided<'a>,
    tensor: &'a TensorBase<T>,
}

impl<'a, T> IndexedIter<'a, T> {
    pub fn new(tensor: &'a TensorBase<T>) -> Self {
        let strides = Strided::from(tensor.layout());
        Self {
            scope: None,
            strides,
            tensor,
        }
    }
}

impl<'a, T> Iterator for IndexedIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let (_pos, idx) = self.strides.next()?;
        self.scope = self.tensor.get_by_index(idx);
        self.scope
    }
}

impl<'a, T> From<&'a TensorBase<T>> for IndexedIter<'a, T> {
    fn from(tensor: &'a TensorBase<T>) -> Self {
        Self::new(tensor)
    }
}
