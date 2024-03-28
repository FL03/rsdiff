/*
    Appellation: tensor <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
// use crate::ops::TrackedOp;
use crate::prelude::{IntoShape, Rank, Shape, TensorId, TensorKind, TensorOp};
use crate::store::Layout;
use acme::prelude::BinaryOp;
use std::ops::Index;
// use std::sync::{Arc, RwLock};

pub(crate) fn new<T>(
    kind: TensorKind,
    op: Option<TensorOp<T>>,
    shape: impl IntoShape,
    store: Vec<T>,
) -> TensorBase<T> {
    TensorBase {
        id: TensorId::new(),
        kind,
        layout: Layout::contiguous(shape),
        op,
        store,
    }
}

pub(crate) fn from_vec<T>(kind: TensorKind, shape: impl IntoShape, store: Vec<T>) -> TensorBase<T> {
    new(kind, None, shape, store)
}

pub(crate) fn from_vec_with_op<T>(
    kind: impl Into<TensorKind>,
    op: TensorOp<T>,
    shape: impl IntoShape,
    store: Vec<T>,
) -> TensorBase<T> {
    new(kind.into(), Some(op), shape, store)
}

#[derive(Clone, Debug)]
// #[derive(Clone, Debug, Eq, Hash, Ord, PartialOrd)]
pub struct TensorBase<T = f64> {
    pub(crate) id: TensorId,
    pub(crate) kind: TensorKind,
    pub(crate) layout: Layout,
    pub(crate) op: Option<TensorOp<T>>,
    pub(crate) store: Vec<T>,
}

impl<T> TensorBase<T> {
    pub fn new(kind: TensorKind, shape: impl IntoShape) -> Self {
        let shape = shape.into_shape();
        let store = Vec::with_capacity(shape.elements());
        Self {
            id: TensorId::new(),
            kind,
            layout: Layout::contiguous(shape),
            op: None,
            store,
        }
    }

    pub fn from_vec(
        kind: TensorKind,
        op: Option<TensorOp<T>>,
        shape: impl IntoShape,
        store: Vec<T>,
    ) -> Self {
        Self {
            id: TensorId::new(),
            kind,
            layout: Layout::contiguous(shape),
            op,
            store,
        }
    }

    pub fn detach(&self) -> Self
    where
        T: Clone,
    {
        if self.op.is_none() && !self.is_variable() {
            self.clone()
        } else {
            Self {
                id: TensorId::new(),
                kind: TensorKind::Normal,
                layout: self.layout.clone(),
                op: None,
                store: self.store.clone(),
            }
        }
    }
    /// Returns the number of elements in the tensor.
    pub fn elements(&self) -> usize {
        self.layout.elements()
    }
    /// Returns the unique identifier of the tensor.
    pub const fn id(&self) -> TensorId {
        self.id
    }
    /// Get a reference to the [Layout] of the tensor
    pub fn layout(&self) -> &Layout {
        &self.layout
    }
    /// Get a reference to the operation of the tensor
    pub fn op(&self) -> Option<&TensorOp<T>> {
        self.op.as_ref()
    }
    /// Get an owned reference to the [Rank] of the tensor
    pub fn rank(&self) -> Rank {
        self.layout.shape().rank()
    }
    /// An owned reference of the tensors [Shape]
    pub fn shape(&self) -> &Shape {
        self.layout.shape()
    }
    /// Get a reference to the stride of the tensor
    pub fn stride(&self) -> &[usize] {
        self.layout.stride()
    }
    /// A function to check if the tensor is a scalar
    pub fn is_scalar(&self) -> bool {
        self.shape().len() == 0
    }
    /// A function to check if the tensor is a variable
    pub const fn is_variable(&self) -> bool {
        self.kind.is_variable()
    }
    /// Changes the kind of tensor to a variable
    pub fn variable(mut self) -> Self {
        self.kind = TensorKind::Variable;
        self
    }
    /// Turn the tensor into a one-dimensional vector
    pub fn to_vec(&self) -> Vec<T>
    where
        T: Clone,
    {
        self.store.clone()
    }

    pub fn apply_binary<F>(&self, op: BinaryOp, other: &Self, f: F) -> Self
    where
        F: Fn(&T, &T) -> T,
        T: Clone,
    {
        let store = self
            .store
            .iter()
            .zip(other.store.iter())
            .map(|(a, b)| f(a, b))
            .collect();
        TensorBase {
            id: TensorId::new(),
            kind: self.kind,
            layout: self.layout.clone(),
            op: Some(TensorOp::Binary(
                Box::new(self.clone()),
                Box::new(other.clone()),
                op,
            )),
            store,
        }
    }

    pub fn map<'a, F>(&'a self, f: F) -> TensorBase<T>
    where
        F: FnMut(&'a T) -> T,
        T: 'a + Clone,
    {
        let store = self.store.iter().map(f).collect();
        TensorBase {
            id: TensorId::new(),
            kind: self.kind,
            layout: self.layout.clone(),
            op: self.op.clone(),
            store,
        }
    }

    pub fn mapv<F>(&self, f: F) -> TensorBase<T>
    where
        F: Fn(T) -> T,
        T: Copy,
    {
        let store = self.store.iter().copied().map(f).collect();
        TensorBase {
            id: TensorId::new(),
            kind: self.kind,
            layout: self.layout.clone(),
            op: self.op.clone(),
            store,
        }
    }
}

impl<T> TensorBase<T> {
    pub(crate) fn data(&self) -> &Vec<T> {
        &self.store
    }
}

impl<T> Index<&[usize]> for TensorBase<T> {
    type Output = T;

    fn index(&self, index: &[usize]) -> &Self::Output {
        let i = self.layout().position(index);
        &self.store[i]
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
