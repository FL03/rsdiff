/*
    Appellation: specs <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::core::cmp::id::AtomicId;
use crate::shape::{Rank, Shape};
use crate::store::Layout;

pub trait NdTensor {
    type Data;

    fn id(&self) -> AtomicId;

    fn layout(&self) -> &Layout;

    fn rank(&self) -> Rank {
        self.layout().shape().rank()
    }

    fn shape(&self) -> &Shape {
        self.layout().shape()
    }

    fn stride(&self) -> &[usize] {
        self.layout().stride()
    }
}
