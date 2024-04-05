/*
    Appellation: ndtensor <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::prelude::{Layout, TensorId};
use crate::shape::{Rank, Shape, Stride};

pub trait NdTensor {
    type Data: TensorData;

    fn id(&self) -> TensorId;

    fn layout(&self) -> &Layout;

    fn rank(&self) -> Rank {
        self.layout().shape().rank()
    }

    fn shape(&self) -> &Shape {
        self.layout().shape()
    }

    fn size(&self) -> usize {
        self.shape().size()
    }

    fn strides(&self) -> &Stride {
        self.layout().strides()
    }
}

pub trait TensorData {
    type Elem;
}

pub trait TensorDataMut: TensorData {
    fn as_mut_ptr(&mut self) -> *mut Self::Elem;
}
