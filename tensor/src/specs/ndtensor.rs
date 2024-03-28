/*
    Appellation: ndtensor <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::prelude::TensorId;
use crate::shape::prelude::{Rank, Shape};
use crate::store::Layout;

pub trait NdTensor {
    type Elem;

    fn id(&self) -> TensorId;

    fn layout(&self) -> &Layout;

    fn rank(&self) -> Rank {
        self.layout().shape().rank()
    }

    fn shape(&self) -> Shape {
        self.layout().shape()
    }

    fn size(&self) -> usize {
        self.shape().size()
    }

    fn stride(&self) -> &[usize] {
        self.layout().stride()
    }
}

pub trait NdStore {
    type Container;
    type Elem;
}

pub trait NdIterator {
    type Item;
}

pub trait NdIndex {
    type Output;
}
