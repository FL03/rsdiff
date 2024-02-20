/*
    Appellation: tensor <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::data::Scalar;
use crate::shape::{IntoShape, Rank, Shape};
use crate::store::Layout;
use acme::cmp::id::AtomicId;
// use std::ops::{Index, IndexMut};
// use std::sync::{Arc, RwLock};

#[derive(Clone, Debug)]
pub struct TensorBase<T> {
    id: AtomicId,
    layout: Layout,
    store: Vec<T>,
}

impl<T> TensorBase<T> {
    pub fn from_vec(shape: impl IntoShape, store: Vec<T>) -> Self {
        let layout = Layout::contiguous(shape);
        Self {
            id: AtomicId::new(),
            layout,
            store, //Arc::new(RwLock::new(store)),
        }
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

    pub fn zeros(shape: impl IntoShape) -> Self {
        Self::fill(shape, T::zero())
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

impl<T> PartialEq for TensorBase<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
