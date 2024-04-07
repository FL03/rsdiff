/*
    Appellation: iterator <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::PositionIter;
use crate::TensorBase;
use core::marker::PhantomData;
use core::ptr::NonNull;

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
    ptr: NonNull<T>,
    scope: Option<&'a mut T>,
    strides: PositionIter,
    tensor: &'a mut TensorBase<T>,
    _marker: PhantomData<&'a mut T>,
}
impl<'a, T> IterMut<'a, T> {
    pub fn new(strides: PositionIter, tensor: &'a mut TensorBase<T>) -> Self {
        let ptr = NonNull::new(tensor.as_mut_ptr()).expect("TensorBase pointer is null");
        Self {
            ptr,
            scope: None,
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
        let scope = unsafe { self.ptr.as_mut() };
        Some(scope)
    }
}
