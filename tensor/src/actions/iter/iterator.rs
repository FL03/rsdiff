/*
    Appellation: iterator <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::LayoutIter;
use crate::TensorBase;
use core::marker::PhantomData;
use core::ptr;

/// An immutable iterator of the elements of a (tensor)[crate::tensor::TensorBase]
/// Elements are visited in order, matching the layout of the tensor.
pub struct Iter<'a, T> {
    inner: LayoutIter,
    ptr: *const T,
    tensor: TensorBase<&'a T>,
}

impl<'a, T> Iter<'a, T> {
    pub fn new(tensor: TensorBase<&'a T>) -> Self {
        Self {
            inner: tensor.layout().iter(),
            ptr: unsafe { *tensor.as_ptr() },
            tensor,
        }
    }
}

impl<'a, T> ExactSizeIterator for Iter<'a, T> {
    fn len(&self) -> usize {
        self.tensor.size()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let pos = self.inner.next()?;
        let item = self.tensor.get_by_index(pos.index())?;
        self.ptr = ptr::from_ref(item);
        unsafe { self.ptr.as_ref() }
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let pos = self.inner.next_back()?;
        let item = self.tensor.get_by_index(pos.index())?;
        self.ptr = ptr::from_ref(item);
        unsafe { self.ptr.as_ref() }
    }
}

impl<'a, T> From<TensorBase<&'a T>> for Iter<'a, T> {
    fn from(tensor: TensorBase<&'a T>) -> Self {
        Self::new(tensor)
    }
}

impl<'a, T> From<&'a TensorBase<T>> for Iter<'a, T> {
    fn from(tensor: &'a TensorBase<T>) -> Self {
        Self::new(tensor.view())
    }
}

pub struct IterMut<'a, T: 'a> {
    inner: LayoutIter,
    ptr: *mut T,
    tensor: &'a mut TensorBase<T>,
    _marker: PhantomData<&'a mut T>,
}

impl<'a, T> IterMut<'a, T> {
    pub(crate) fn new(tensor: &'a mut TensorBase<T>) -> Self {
        Self {
            inner: tensor.layout().iter(),
            ptr: tensor.as_mut_ptr(),
            tensor,
            _marker: PhantomData,
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let pos = self.inner.next()?;
        let elem = self.tensor.get_mut_by_index(pos.index())?;
        self.ptr = ptr::from_mut(elem);
        unsafe { self.ptr.as_mut() }
    }
}

impl<'a, T> ExactSizeIterator for IterMut<'a, T> {
    fn len(&self) -> usize {
        self.tensor.size()
    }
}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let pos = self.inner.next_back()?;
        let elem = self.tensor.get_mut_by_index(pos.index())?;

        self.ptr = ptr::from_mut(elem);
        unsafe { self.ptr.as_mut() }
    }
}
