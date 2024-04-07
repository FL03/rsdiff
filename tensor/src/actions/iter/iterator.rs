/*
    Appellation: iterator <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::PositionIter;
use crate::TensorBase;
use core::marker::PhantomData;
use core::ptr::{self, NonNull};

pub struct Iter<'a, T> {
    pos: PositionIter,
    scope: Option<&'a T>,
    tensor: &'a TensorBase<T>,
}

impl<'a, T> Iter<'a, T> {
    pub fn new(tensor: &'a TensorBase<T>) -> Self {
        let pos = PositionIter::from(tensor.layout());
        Self {
            pos,
            scope: None,
            tensor,
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let (_pos, idx) = self.pos.next()?;
        self.scope = self.tensor.get_by_index(idx);
        self.scope
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let (_pos, idx) = self.pos.next_back()?;
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
    ptr: *mut T,
    strides: PositionIter,
    tensor: &'a mut TensorBase<T>,
    _marker: PhantomData<&'a mut T>,
}
impl<'a, T> IterMut<'a, T> {
    pub(crate) fn new(tensor: &'a mut TensorBase<T>) -> Self {
        // let ptr = NonNull::new(tensor.as_mut_ptr()).expect("TensorBase pointer is null");
        let strides = PositionIter::from(tensor.layout());
        let ptr = tensor.as_mut_ptr();
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
        let (_pos, idx) = self.strides.next()?;
        let elem = self.tensor.get_mut_by_index(idx)?;

        self.ptr = ptr::from_mut(elem);
        unsafe { self.ptr.as_mut() }
    }
}
