/*
    Appellation: axis <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::index::{Ix, Ixs};
use crate::shape::{Axis, Layout};
use crate::TensorBase;

pub struct AxisIter<A> {
    index: Ix,
    end: Ix,
    stride: Ixs,
    inner_layout: Layout,
    ptr: *mut A,
}

impl<A> AxisIter<A> {
    pub fn new<S>(v: TensorBase<S>, axis: Axis) -> Self {
        let stride = v.strides()[axis];
        let end = v.shape()[axis];
        // Self {
        //     index: 0,
        //     end,
        //     stride,
        //     inner_layout: layout.remove_axis(axis),
        //     ptr: v.as_mut_ptr(),
        // }
        unimplemented!()
    }
}

impl<A> Iterator for AxisIter<A> {
    type Item = *mut A;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.end {
            return None;
        }
        let ptr = unsafe { self.ptr.add(self.index as usize) };
        self.index += self.stride as Ix;
        Some(ptr)
    }
}
