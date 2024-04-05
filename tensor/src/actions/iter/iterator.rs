/*
    Appellation: iterator <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::IndexIter;
use crate::TensorBase;
use core::marker::PhantomData;
use core::ptr::NonNull;
pub struct Iter<'a, T> {
    scope: Option<&'a T>,
    strides: IndexIter,
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

pub struct IterMut<'a, T: 'a> {
    ptr: NonNull<T>,
    strides: IndexIter,
    tensor: &'a mut TensorBase<T>,
    _marker: PhantomData<&'a mut T>,
}
impl<'a, T> IterMut<'a, T> {
    pub fn new(strides: IndexIter, tensor: &'a mut TensorBase<T>) -> Self {
        let ptr = NonNull::new(tensor.as_mut_ptr()).expect("TensorBase pointer is null");
        Self {
            ptr,
            strides,
            tensor,
            _marker: PhantomData,
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let (_pos, _idx) = self.strides.next()?;
        unimplemented!()
    }
}
