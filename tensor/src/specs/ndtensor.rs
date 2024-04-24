/*
    Appellation: ndtensor <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::prelude::{Layout, TensorId};
use crate::shape::{Rank, Shape, Stride};

pub trait NdTensor<T> {
    type Data: TensorData<Elem = T>;

    fn as_mut_ptr(&mut self) -> *mut T;

    fn as_ptr(&self) -> *const T;

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

impl<T> TensorData for [T] {
    type Elem = T;
}

impl<'a, T> TensorData for &'a [T] {
    type Elem = T;
}

impl<T> TensorData for Vec<T> {
    type Elem = T;
}

pub trait TensorDataMut: TensorData {
    fn as_mut_ptr(&mut self) -> *mut Self::Elem;
}
