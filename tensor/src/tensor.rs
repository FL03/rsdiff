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
    from_vec_with_op(None, shape, store)
}

pub(crate) fn from_vec_with_op<T>(
    op: Option<Op<T>>,
    shape: impl IntoShape,
    store: Vec<T>,
) -> TensorBase<T> {
    let layout = Layout::contiguous(shape);
    TensorBase {
        id: AtomicId::new(),
        layout,
        op,
        store, //Arc::new(RwLock::new(store)),
    }
}

#[derive(Clone, Debug)]
pub struct TensorBase<T> {
    id: AtomicId,
    layout: Layout,
    op: Option<Op<T>>,
    store: Vec<T>,
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

    pub fn zeros(shape: impl IntoShape) -> Self {
        Self::fill(shape, T::zero())
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
        from_vec_with_op(Some(op), shape, result)
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
        self.id == other.id || self.store == other.store
    }
}

macro_rules! cmp {
    (ne: $lhs:expr, $rhs:expr) => {
        if $lhs != $rhs {
            panic!("Shape Mismatch: {:?} != {:?}", $lhs, $rhs);
        }
    };
}

macro_rules! impl_arith {
    ($trait:ident, $method:ident, $op:tt) => {
        impl<T> std::ops::$trait for TensorBase<T>
        where
            T: Scalar + std::ops::$trait<Output = T>,
        {
            type Output = Self;

            fn $method(self, other: Self) -> Self::Output {
                cmp!(ne: self.shape(), other.shape());
                let shape = self.shape().clone();
                let store = self.store.iter().zip(other.store.iter()).map(|(a, b)| *a $op *b).collect();
                let op = Op::Binary(Box::new(self), Box::new(other), BinaryOp::$trait);
                from_vec_with_op(Some(op), shape, store)
            }
        }

        impl<'a, T> std::ops::$trait<&'a TensorBase<T>> for TensorBase<T>
        where
            T: Scalar + std::ops::$trait<Output = T>,
        {
            type Output = TensorBase<T>;

            fn $method(self, other: &'a TensorBase<T>) -> Self::Output {
                if self.shape() != other.shape() {
                    panic!("shapes must be equal");
                }
                let shape = self.shape().clone();
                let store = self.store.iter().zip(other.store.iter()).map(|(a, b)| *a $op *b).collect();
                let op = Op::Binary(Box::new(self), Box::new(other.clone()), BinaryOp::$trait);
                from_vec_with_op(Some(op), shape, store)
            }
        }

        impl<'a, T> std::ops::$trait<TensorBase<T>> for &'a TensorBase<T>
        where
            T: Scalar + std::ops::$trait<Output = T>,
        {
            type Output = TensorBase<T>;

            fn $method(self, other: TensorBase<T>) -> Self::Output {
                if self.shape() != other.shape() {
                    panic!("shapes must be equal");
                }
                let shape = self.shape().clone();
                let store = self.store.iter().zip(other.store.iter()).map(|(a, b)| *a $op *b).collect();
                let op = Op::Binary(Box::new(self.clone()), Box::new(other), BinaryOp::$trait);
                from_vec_with_op(Some(op), shape, store)
            }
        }

        impl<'a, 'b, T> std::ops::$trait<&'b TensorBase<T>> for &'a TensorBase<T>
        where
            T: Scalar + std::ops::$trait<Output = T>,
        {
            type Output = TensorBase<T>;

            fn $method(self, other: &'b TensorBase<T>) -> Self::Output {
                if self.shape() != other.shape() {
                    panic!("shapes must be equal");
                }
                let shape = self.shape().clone();
                let store = self.store.iter().zip(other.store.iter()).map(|(a, b)| *a $op *b).collect();
                let op = Op::Binary(Box::new(self.clone()), Box::new(other.clone()), BinaryOp::$trait);
                from_vec_with_op(Some(op), shape, store)
            }
        }
    };
}

macro_rules! impl_scalar_arith {
    ($trait:ident, $method:ident, $op:tt) => {
        // impl<T> TensorBase<T>
        // where
        //     T: Copy + std::ops::$trait<Output = T>,
        // {
        //     pub fn $method(self, other: T) -> TensorBase<T> {
        //         let store = self.store.iter().map(|a| *a $op other).collect();
        //         from_vec(self.shape().clone(), store)
        //     }
        // }

        impl<T> std::ops::$trait<T> for TensorBase<T>
        where
            T: Copy + std::ops::$trait<Output = T>,
        {
            type Output = Self;

            fn $method(self, other: T) -> Self::Output {
                let store = self.store.iter().map(|a| *a $op other).collect();
                Self::Output::from_vec(self.shape().clone(), store)
            }
        }

        impl<'a, T> std::ops::$trait<T> for &'a TensorBase<T>
        where
            T: Copy + std::ops::$trait<Output = T>,
        {
            type Output = TensorBase<T>;

            fn $method(self, other: T) -> Self::Output {
                let store = self.store.iter().map(|a| *a $op other).collect();
                Self::Output::from_vec(self.shape().clone(), store)
            }
        }
    };
}

impl_arith!(Add, add, +);
impl_arith!(Div, div, /);
impl_arith!(Mul, mul, *);
impl_arith!(Sub, sub, -);

impl_scalar_arith!(Add, add, +);
impl_scalar_arith!(Div, div, /);
impl_scalar_arith!(Mul, mul, *);
impl_scalar_arith!(Sub, sub, -);
