/*
    Appellation: axis <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::data::{ContainerBase, RawData};
use crate::index::{Ix, Ixs};
use crate::shape::{Axis, Layout};

pub struct AxisIter<A> {
    index: Ix,
    end: Ix,
    stride: Ixs,
    inner_layout: Layout,
    ptr: *mut A,
}

impl<A> AxisIter<A> {
    pub fn new<S>(v: ContainerBase<S>, axis: Axis) -> Self
    where
        S: RawData<Elem = A>,
    {
        let stride = v.stride()[axis];
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
