/*
    Appellation: stride <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::shape::{Shape, Stride};
use crate::store::Layout;
use crate::tensor::TensorBase;

pub struct StrideIter<'a, T> {
    scope: Option<&'a T>,
    strides: Strides<'a>,
    tensor: &'a TensorBase<T>,
}

impl<'a, T> StrideIter<'a, T> {
    pub fn new(tensor: &'a TensorBase<T>) -> Self {
        let strides = Strides::from(tensor.layout());
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
        let idx = self.strides.next()?;
        self.scope = self.tensor.get_by_index(idx);
        self.scope
    }
}

pub struct Strides<'a> {
    next: Option<usize>,
    position: Vec<usize>,
    pub(crate) shape: &'a Shape,
    pub(crate) stride: &'a Stride,
}

impl<'a> Strides<'a> {
    pub fn new(offset: usize, shape: &'a Shape, stride: &'a Stride) -> Self {
        let elem_count: usize = shape.iter().product();
        let next = if elem_count == 0 {
            None
        } else {
            // This applies to the scalar case.
            Some(offset)
        };
        Self {
            next,
            position: vec![0; *shape.rank()],
            shape,
            stride,
        }
    }

    pub fn index(&self, index: &[usize]) -> usize {
        index
            .iter()
            .zip(self.stride.iter())
            .map(|(i, s)| i * s)
            .sum()
    }
}

impl<'a> Iterator for Strides<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let scope = match self.next {
            None => return None,
            Some(storage_index) => storage_index,
        };
        let mut updated = false;
        let mut next = scope;
        for ((multi_i, max_i), stride_i) in self
            .position
            .iter_mut()
            .zip(self.shape.iter())
            .zip(self.stride.iter())
            .rev()
        {
            let next_i = *multi_i + 1;
            if next_i < *max_i {
                *multi_i = next_i;
                updated = true;
                next += stride_i;
                break;
            } else {
                next -= *multi_i * stride_i;
                *multi_i = 0
            }
        }
        self.next = if updated { Some(next) } else { None };
        Some(scope)
    }
}

impl<'a> From<&'a Layout> for Strides<'a> {
    fn from(layout: &'a Layout) -> Self {
        Self::new(layout.offset, &layout.shape, &layout.stride)
    }
}
