/*
    Appellation: tensor <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::ops::kinds::{BinaryOp, Op};
use crate::prelude::Scalar;
use crate::shape::{IntoShape, Rank, Shape};
use crate::store::Layout;
use acme::prelude::AtomicId;
// use std::ops::{Index, IndexMut};
// use std::sync::{Arc, RwLock};

pub(crate) fn from_vec<T>(shape: impl IntoShape, store: Vec<T>) -> TensorBase<T> {
    TensorBase {
        id: AtomicId::new(),
        layout: Layout::contiguous(shape),
        op: None,
        store, //Arc::new(RwLock::new(store)),
    }
}

pub(crate) fn from_vec_with_op<T>(
    op: Op<T>,
    shape: impl IntoShape,
    store: Vec<T>,
) -> TensorBase<T> {
    let layout = Layout::contiguous(shape);
    TensorBase {
        id: AtomicId::new(),
        layout,
        op: Some(op),
        store, //Arc::new(RwLock::new(store)),
    }
}

#[derive(Clone, Debug)]
pub struct TensorBase<T> {
    pub(crate) id: AtomicId,
    pub(crate) layout: Layout,
    pub(crate) op: Option<Op<T>>,
    pub(crate) store: Vec<T>,
}

impl<T> TensorBase<T> {
    pub fn from_vec(shape: impl IntoShape, store: Vec<T>) -> Self {
        from_vec(shape, store)
    }

    // Function to get the index of the data based on coordinates
    fn position(&self, coords: impl AsRef<[usize]>) -> usize {
        self.layout.position(coords.as_ref())
    }

    pub fn id(&self) -> usize {
        self.id.get()
    }

    pub fn layout(&self) -> &Layout {
        &self.layout
    }

    pub fn op(&self) -> Option<&Op<T>> {
        self.op.as_ref()
    }

    pub fn rank(&self) -> Rank {
        self.layout.shape().rank()
    }

    pub fn shape(&self) -> &Shape {
        self.layout.shape()
    }

    pub fn stride(&self) -> &[usize] {
        self.layout.stride()
    }
}

impl<T> TensorBase<T>
where
    T: Clone,
{
    pub fn empty(shape: impl IntoShape) -> Self
    where
        T: Default,
    {
        Self::fill(shape, T::default())
    }

    pub fn fill(shape: impl IntoShape, value: T) -> Self {
        let shape = shape.into_shape();
        let store = vec![value; shape.elements()];
        Self::from_vec(shape, store)
    }
}

impl<T> TensorBase<T>
where
    T: Scalar,
{
    pub fn arange(start: T, end: T, step: T) -> Self
    where
        T: PartialOrd,
    {
        if T::is_zero(&step) {
            panic!("step must be non-zero");
        }

        let mut store = vec![start];
        let mut cur = T::zero();
        while store.last().unwrap() < &end {
            cur += step;
            store.push(cur);
        }
        Self::from_vec(store.len(), store)
    }

    pub fn ones(shape: impl IntoShape) -> Self {
        Self::fill(shape, T::one())
    }

    pub fn ones_like(tensor: &TensorBase<T>) -> Self {
        Self::ones(tensor.shape().clone())
    }

    pub fn zeros(shape: impl IntoShape) -> Self {
        Self::fill(shape, T::zero())
    }

    pub fn zeros_like(tensor: &TensorBase<T>) -> Self {
        Self::zeros(tensor.shape().clone())
    }
}

impl<T> TensorBase<T>
where
    T: Scalar,
{
    pub fn matmul(&self, other: &Self) -> Self {
        let shape = self.shape().matmul_shape(other.shape()).unwrap();
        let mut result = vec![T::zero(); shape.elements()];

        for i in 0..self.shape()[0] {
            for j in 0..other.shape()[1] {
                for k in 0..self.shape()[1] {
                    result[i * other.shape()[1] + j] +=
                        self.store[i * self.shape()[1] + k] * other.store[k * other.shape()[1] + j];
                }
            }
        }
        let op = Op::Binary(
            Box::new(self.clone()),
            Box::new(other.clone()),
            BinaryOp::Matmul,
        );
        from_vec_with_op(op, shape, result)
    }
}

impl<T> std::ops::Index<&[usize]> for TensorBase<T> {
    type Output = T;

    fn index(&self, index: &[usize]) -> &Self::Output {
        &self.store[self.position(index)]
    }
}

// impl<T> IndexMut<&[usize]> for Tensor<T> {
//     fn index_mut(&mut self, index: &[usize]) -> &mut Self::Output {
//         self.get_mut(index).unwrap()
//     }
// }

impl<T> Eq for TensorBase<T> where T: Eq {}

impl<T> PartialEq for TensorBase<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.store == other.store
    }
}

impl<T> num::traits::One for TensorBase<T>
where
    T: Scalar,
{
    fn one() -> Self {
        Self::fill(1, T::one())
    }
}
