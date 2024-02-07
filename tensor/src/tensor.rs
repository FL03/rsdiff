/*
    Appellation: tensor <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::core::cmp::id::AtomicId;
use crate::data::Scalar;
use crate::shape::{IntoShape, Rank, Shape};
use crate::store::Layout;
use std::ops::{Index, IndexMut};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Tensor<T> {
    id: AtomicId,
    layout: Layout,
    store: Vec<T>,
}

impl<T> Tensor<T> {
    pub fn from_vec(shape: impl IntoShape, store: Vec<T>) -> Self {
        let id = AtomicId::new();
        let layout = Layout::contiguous(shape);
        Self { id, layout, store }
    }

    // Function to get the index of the data based on coordinates
    fn index(&self, coords: &[usize]) -> usize {
        let mut index = self.layout.offset;
        for (i, &coord) in coords.iter().enumerate() {
            index += coord * self.layout.stride[i];
        }
        index
    }

    // Function to get a reference to an element at given coordinates
    fn get(&self, coords: &[usize]) -> Option<&T> {
        let index = self.index(coords);
        self.store.get(index)
    }

    // Function to get a mutable reference to an element at given coordinates
    fn get_mut(&mut self, coords: &[usize]) -> Option<&mut T> {
        let index = self.index(coords);
        self.store.get_mut(index)
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

impl<T> Tensor<T>
where
    T: Clone + Default,
{
    pub fn empty(shape: impl IntoShape) -> Self {
        let id = AtomicId::new();
        let layout = Layout::contiguous(shape);
        let store = vec![T::default(); layout.elements()];
        Self { id, layout, store }
    }

    pub fn fill(shape: impl IntoShape, value: T) -> Self {
        let id = AtomicId::new();
        let layout = Layout::contiguous(shape);
        let store = vec![value; layout.elements()];
        Self { id, layout, store }
    }
}

impl<T> Tensor<T>
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

        let id = AtomicId::new();
        let mut store = vec![start];
        let layout = Layout::contiguous(store.len());
        let mut cur = T::zero();
        while store.last().unwrap() < &end {
            cur += step;
            store.push(cur);
        }
        Self { id, layout, store }
    }

    pub fn ones(shape: impl IntoShape) -> Self {
        let id = AtomicId::new();
        let layout = Layout::contiguous(shape);
        let store = vec![T::one(); layout.elements()];
        Self { id, layout, store }
    }

    pub fn zeros(shape: impl IntoShape) -> Self {
        let id = AtomicId::new();
        let layout = Layout::contiguous(shape);
        let store = vec![T::zero(); layout.elements()];
        Self { id, layout, store }
    }
}

impl<T> Index<&[usize]> for Tensor<T> {
    type Output = T;

    fn index(&self, index: &[usize]) -> &Self::Output {
        self.get(index).unwrap()
    }
}
