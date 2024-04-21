/*
    Appellation: indexed <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::iter::LayoutIter;
use crate::tensor::TensorBase;

use super::Position;

pub struct IndexedIter<'a, T: 'a> {
    inner: LayoutIter,
    scope: Option<&'a T>,
    tensor: &'a TensorBase<T>,
}

impl<'a, T> IndexedIter<'a, T> {
    pub fn new(tensor: &'a TensorBase<T>) -> Self {
        Self {
            inner: tensor.layout().iter(),
            scope: None,
            tensor,
        }
    }
}

impl<'a, T> Iterator for IndexedIter<'a, T> {
    type Item = (&'a T, Position);

    fn next(&mut self) -> Option<Self::Item> {
        let pos = self.inner.next()?;
        self.scope = self.tensor.get_by_index(pos.index());
        self.scope.map(|scope| (scope, pos))
    }
}

impl<'a, T> DoubleEndedIterator for IndexedIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let pos = self.inner.next_back()?;
        self.scope = self.tensor.get_by_index(pos.index());
        self.scope.map(|scope| (scope, pos))
    }
}

impl<'a, T> ExactSizeIterator for IndexedIter<'a, T> {
    fn len(&self) -> usize {
        self.tensor.size()
    }
}
