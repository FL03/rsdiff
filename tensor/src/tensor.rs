/*
    Appellation: tensor <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
// use crate::ops::TrackedOp;
use crate::prelude::{IntoShape, Rank, Shape, TensorId, TensorKind, TensorOp};
use crate::store::Layout;
use std::ops::Index;
// use std::sync::{Arc, RwLock};

pub(crate) fn from_vec<T>(kind: TensorKind, shape: impl IntoShape, store: Vec<T>) -> TensorBase<T> {
    TensorBase {
        id: TensorId::new(),
        kind,
        layout: Layout::contiguous(shape),
        op: None,
        store,
    }
}

pub(crate) fn from_vec_with_op<T>(
    kind: impl Into<TensorKind>,
    op: TensorOp<T>,
    shape: impl IntoShape,
    store: Vec<T>,
) -> TensorBase<T> {
    let layout = Layout::contiguous(shape);
    TensorBase {
        id: TensorId::new(),
        kind: kind.into(),
        layout,
        op: Some(op),
        store,
    }
}

#[derive(Clone, Debug)]
// #[derive(Clone, Debug, Eq, Hash, Ord, PartialOrd)]
pub struct TensorBase<T> {
    pub(crate) id: TensorId,
    pub(crate) kind: TensorKind,
    pub(crate) layout: Layout,
    pub(crate) op: Option<TensorOp<T>>,
    pub(crate) store: Vec<T>,
}

impl<T> TensorBase<T> {
    pub fn new(kind: TensorKind, shape: impl IntoShape) -> Self {
        Self {
            id: TensorId::new(),
            kind,
            layout: Layout::contiguous(shape),
            op: None,
            store: Vec::new(),
        }
    }

    pub fn from_vec(kind: TensorKind, shape: impl IntoShape, store: Vec<T>) -> Self {
        from_vec(kind, shape, store)
    }
    /// Returns the number of elements in the tensor.
    pub fn elements(&self) -> usize {
        self.layout.elements()
    }
    /// Returns the unique identifier of the tensor.
    pub fn id(&self) -> TensorId {
        self.id
    }
    /// Get a reference to the layout of the tensor
    pub fn layout(&self) -> &Layout {
        &self.layout
    }

    pub fn op(&self) -> Option<&TensorOp<T>> {
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

    pub fn is_variable(&self) -> bool {
        self.kind.is_variable()
    }

    pub fn variable(mut self) -> Self {
        self.kind = TensorKind::Variable;
        self
    }
    pub fn to_vec(&self) -> Vec<T>
    where
        T: Clone,
    {
        self.store.clone()
    }
}

impl<T> TensorBase<T> {
    pub(crate) fn data(&self) -> &Vec<T> {
        &self.store
    }
    // An internal function to get the index of the data based on coordinates
    pub(crate) fn position(&self, coords: impl AsRef<[usize]>) -> usize {
        self.layout.position(coords.as_ref())
    }
}

impl<T> Index<&[usize]> for TensorBase<T> {
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
