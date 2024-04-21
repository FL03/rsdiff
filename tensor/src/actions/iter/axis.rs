/*
    Appellation: axis <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::index::{Ix, Ixs};
use crate::shape::{Axis, Layout};
use crate::TensorBase;
use core::ptr;

pub struct AxisIter<'a, A> {
    index: Ix,
    end: Ix,
    stride: Ixs,
    inner_layout: Layout,
    ptr: *const A,
    tensor: TensorBase<&'a A>,
}

impl<'a, A> AxisIter<'a, A> {
    pub fn new(v: TensorBase<&'a A>, axis: Axis) -> Self {
        let stride = v.strides()[axis] as isize;
        let end = v.shape()[axis];
        Self {
            index: 0,
            end,
            stride,
            inner_layout: v.layout().remove_axis(axis),
            ptr: unsafe { *v.as_ptr() },
            tensor: v,
        }
    }
}

impl<'a, A> Iterator for AxisIter<'a, A> {
    type Item = &'a A;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.end {
            return None;
        }
        let ptr = unsafe { self.ptr.add(self.index) };
        self.index += self.stride as Ix;
        unsafe { ptr.as_ref() }
    }
}
