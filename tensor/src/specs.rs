/*
    Appellation: specs <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::shape::{Rank, Shape};
use crate::store::Layout;
use acme::prelude::AtomicId;

pub trait Affine<T> {
    type Output;

    fn affine(&self, mul: &T, add: &T) -> Self::Output;
}

pub trait Matmul<Rhs = Self> {
    type Output;

    fn matmul(&self, rhs: &Rhs) -> Self::Output;
}

pub trait NdTensor {
    fn elements(&self) -> usize {
        self.layout().elements()
    }

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
