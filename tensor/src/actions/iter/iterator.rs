/*
    Appellation: iterator <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::IndexIter;
use crate::TensorBase;

pub struct Iter<'a, T> {
    scope: Option<&'a T>,
    strides: IndexIter<'a>,
    tensor: &'a TensorBase<T>,
}

impl<'a, T> Iter<'a, T> {
    pub fn new(tensor: &'a TensorBase<T>) -> Self {
        let strides = IndexIter::from(tensor.layout());
        Self {
            scope: None,
            strides,
            tensor,
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let (_pos, idx) = self.strides.next()?;
        self.scope = self.tensor.get_by_index(idx);
        self.scope
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let (_pos, idx) = self.strides.next_back()?;
        self.scope = self.tensor.get_by_index(idx);
        self.scope
    }
}

impl<'a, T> From<&'a TensorBase<T>> for Iter<'a, T> {
    fn from(tensor: &'a TensorBase<T>) -> Self {
        Self::new(tensor)
    }
}

#[allow(dead_code)]
pub struct IterMut<'a, T> {
    scope: Option<&'a mut T>,
    strides: IndexIter<'a>,
    tensor: &'a mut TensorBase<T>,
}

impl<'a, T> IterMut<'a, T> {
    pub fn new(strides: IndexIter<'a>, tensor: &'a mut TensorBase<T>) -> Self {
        Self {
            scope: None,
            strides,
            tensor,
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let (_pos, idx) = self.strides.next()?;
        unimplemented!()
    }
}
