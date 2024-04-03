/*
    Appellation: iterator <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::IndexIter;
use crate::TensorBase;

pub struct StrideIter<'a, T> {
    scope: Option<&'a T>,
    strides: IndexIter<'a>,
    tensor: &'a TensorBase<T>,
}

impl<'a, T> StrideIter<'a, T> {
    pub fn new(tensor: &'a TensorBase<T>) -> Self {
        let strides = IndexIter::from(tensor.layout());
        Self {
            scope: None,
            strides,
            tensor,
        }
    }
}

impl<'a, T> Iterator for StrideIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let (_pos, idx) = self.strides.next()?;
        self.scope = self.tensor.get_by_index(idx);
        self.scope
    }
}

impl<'a, T> DoubleEndedIterator for StrideIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let (_pos, idx) = self.strides.next_back()?;
        self.scope = self.tensor.get_by_index(idx);
        self.scope
    }
}

impl<'a, T> From<&'a TensorBase<T>> for StrideIter<'a, T> {
    fn from(tensor: &'a TensorBase<T>) -> Self {
        Self::new(tensor)
    }
}
